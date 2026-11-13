import AppKit
import QuartzCore

// AppKit host only. Rendering, hover animation, camera and geographic painting
// remain in the vendored EndfieldHUD classes.
private final class AtelierHudView: NSView {
    override var isFlipped: Bool { true }
    let navigation: HUDNavigation
    let map: WorldMapSurface
    let artwork = HUDMechanicalArtwork()
    let identityCard = HUDIdentityCard()
    let headerClock = HUDClock()
    let accountGauge = HUDAccountGauge()
    let chargeBadge = HUDChargeBadge()
    let battery = BatteryMonitor()
    private var batterySnapshot: BatterySnapshot = .unavailable
    let header = CALayer()
    private let profileStore = Result { try UserProfileStore(directory: FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0].appendingPathComponent("com.codexatelier.desktop/HUD/Profile")) }
    private var peripheralView: SystemHUDView?
    private var profileObserver: UUID?
    private let workMode = WorkModeController()
    private lazy var account = HypergryphAccountController(profile: try? profileStore.get(), fileURL: FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0].appendingPathComponent("com.codexatelier.desktop/HUD/Account/profile-cache.json"))
    private var accountObserver: UUID?

    private func refreshProfile() {
        guard let store = try? profileStore.get() else { return }
        identityCard.setProfile(store.profile, avatar: store.image(for: .avatar), background: store.image(for: .background), avatarOrientation: store.imageOrientation(for: .avatar))
    }
    private func refreshAccount() {
        let value = account.gaugeValue(work: workMode.snapshot)
        accountGauge.update(value: value.0, accessibilityLabel: value.1, visible: true, accent: .yellow, scale: 2, sanity: account.sanityPresentation())
    }
    private func openPeripheral(_ module: HUDModule, quit: Bool = false) {
        if peripheralView == nil {
            let root = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0].appendingPathComponent("com.codexatelier.desktop/HUD")
            let view = SystemHUDView(frame: bounds,
                notesStore: Result { try NotesStore(directory: root.appendingPathComponent("Notes")) },
                shelfStore: Result { try FileShelfStore(directory: root.appendingPathComponent("Shelf")) },
                clipboard: ClipboardWatcher(store: ClipboardStore(), pasteboard: .withUniqueName()),
                audio: .fixture(), perAppAudio: .fixture(), workMode: workMode,
                appShortcuts: Result { try AppShortcutStore(directory: root.appendingPathComponent("Shortcuts")) },
                profile: profileStore,
                mapStore: Result { try WorldMapStore(directory: root.appendingPathComponent("Map")) },
                archive: ArchiveController(store: ArchiveStore(directory: root.appendingPathComponent("Archive"))),
                reader: ReaderController(loadStore: { try ReaderStore(directory: root.appendingPathComponent("Reader")) }),
                account: account, initialModule: module)
            view.onClose = { [weak self, weak view] in
                view?.interactionEnabled = false
                view?.stopMotionForConcealment()
                view?.isHidden = true
                if let self { self.setAccessibilityChildren(self.actions) }
                self?.scene.isHidden = false
                self?.refreshProfile()
            }
            view.onQuitConfirmed = { NSApp.terminate(nil) }
            addSubview(view)
            peripheralView = view
        }
        guard let view = peripheralView else { return }
        view.frame = bounds
        scene.isHidden = true
        setAccessibilityChildren([view])
        view.set(snapshot: batterySnapshot, configuration: .defaults)
        view.isHidden = false
        view.interactionEnabled = true
        view.showStable()
        view.selectModule(module, animated: false)
        window?.makeFirstResponder(view)
        if quit { view.presentQuitConfirmation() }
    }


    let plane = HUDDepthPlane(name: "atelier", depth: 0, travel: 8, lag: 0.28)
    let scene = CALayer()
    lazy var motion = HUDMotionController(planes: [plane])
    // Index ↔ destination mapping. This order must match `nav` in
    // src/app/navigation.ts one-to-one: the web layer sends labels by index and
    // routes the index it gets back. The six fixed tiles are left/bottom slots;
    // the right-hand column keeps the first contiguous cards so no holes appear
    // after the demo extensions, diagnostics and persona entries were removed.
    let modules: [HUDModule] = [.system, .display, .hotkeys, .about, .storage, .activityMonitor,
                                .notes, .fileShelf, .clipboard, .archive, .mediaAssembly]
    var onRoute: (Int32) -> Void = { _ in }
    var focusIndex = 0
    var actions: [AtelierHudAction] = []
    override var acceptsFirstResponder: Bool { true }
    var tracking: NSTrackingArea?
    var scale: CGFloat = 1
    var origin = CGPoint.zero
    let mapScale = 208 / WorldMapSurface.radius
    var mapOrigin: CGPoint { CGPoint(x: 500 - WorldMapSurface.size.width * mapScale / 2,
                                    y: 320 - WorldMapSurface.size.height * mapScale / 2) }

    init(labels: [String], terrain: WorldMapTerrain, countries: WorldMapCountries) {
        navigation = HUDNavigation(prepareArtwork: true, allowedModules: Set(modules))
        map = WorldMapSurface(store: nil, terrain: terrain, countries: countries, loadsTerrain: false,
                              accent: NSColor(red: 1, green: 0.98, blue: 0, alpha: 1).cgColor,
                              reduceMotion: { HUDRuntimeAppearance.reduceMotion }, ambient: { true })
        super.init(frame: .zero)
        wantsLayer = true
        layer!.backgroundColor = NSColor(red: 0.10, green: 0.12, blue: 0.11, alpha: 1).cgColor
        scene.bounds = CGRect(x: 0, y: 0, width: 1000, height: 640)
        scene.anchorPoint = .zero
        layer!.addSublayer(scene)
        scene.addSublayer(plane.deployment)
        let content = plane.content
        [artwork.distant, artwork.rear, artwork.secondary, artwork.frame, artwork.inner].forEach(content.addSublayer)
        map.layer.anchorPoint = .zero
        map.layer.position = mapOrigin
        map.layer.setAffineTransform(CGAffineTransform(scaleX: mapScale, y: mapScale))
        content.addSublayer(map.layer)
        [artwork.markers, artwork.glass, artwork.rim, navigation.layer, navigation.bottomLayer].forEach(content.addSublayer)
        // Upstream SystemHUDView.buildHeader/buildNavigation composition;
        // component artwork and 1000×640 design coordinates are unchanged.
        content.addSublayer(identityCard.layer)
        identityCard.update(dark: true, accent: .yellow, contentsScale: 2)
        header.frame = HUDOverlayGeometry.header
        header.zPosition = 20
        content.addSublayer(header)
        let titleLabel = HUDStandardTextLayer(size: 20, weight: .semibold)
        titleLabel.frame = CGRect(x: 0, y: 0, width: 300, height: 27)
        titleLabel.string = "ENDFIELDHUD"
        let subtitleLabel = HUDStandardTextLayer(size: 9)
        subtitleLabel.frame = CGRect(x: 0, y: 31, width: 300, height: 18)
        subtitleLabel.string = "SYSTEM INTERFACE"
        let clockTime = HUDStandardTextLayer(size: 20, weight: .semibold, tabular: true)
        clockTime.frame = CGRect(x: 340, y: 0, width: 220, height: 27)
        clockTime.alignmentMode = .right
        let clockDate = HUDStandardTextLayer(size: 9, tabular: true)
        clockDate.frame = CGRect(x: 340, y: 31, width: 220, height: 18)
        clockDate.alignmentMode = .right
        for label in [titleLabel, subtitleLabel, clockTime, clockDate] {
            label.foregroundColor = NSColor.white.cgColor
            header.addSublayer(label)
        }
        headerClock.onChange = { reading in
            clockTime.string = reading.time
            clockDate.string = reading.date
        }
        accountGauge.layer.position = HUDAccountGauge.headerPosition
        header.addSublayer(accountGauge.layer)
        accountGauge.update(value: "—", accessibilityLabel: "未连接账号", visible: true, accent: .yellow, scale: 2)
        content.addSublayer(chargeBadge.layer)
        battery.onChange = { [weak self] snapshot in
            self?.batterySnapshot = snapshot
            self?.peripheralView?.set(snapshot: snapshot, configuration: .defaults)
            self?.chargeBadge.update(snapshot: snapshot, configuration: .defaults, dark: true, contentsScale: 2)
        }
        artwork.update(dark: true, chargeColor: .yellow, accentColor: .yellow)
        navigation.update(dark: true, accent: NSColor(red: 1, green: 0.98, blue: 0, alpha: 1), contentsScale: 2)
        for entry in navigation.entries {
            guard let module = entry.module, let index = modules.firstIndex(of: module) else { continue }
            entry.setTitle(labels[index])
        }
        for (index, rotor) in ([artwork.rearRotor, artwork.secondaryRotor, artwork.innerGuideRotor] + artwork.triangleRotors).enumerated() {
            motion.registerAmbient(layer: rotor, key: "ring\(index)", keyPath: "transform.rotation.z", fromValue: 0,
                                   toValue: HUDMechanicalArtwork.watchRingExcursion,
                                   duration: HUDMechanicalArtwork.watchLoopLegDuration, timingFunction: .linear)
        }
        motion.registerAmbient(layer: artwork.meshRotor, key: "mesh", keyPath: "transform.rotation.z", fromValue: 0,
                               toValue: HUDMechanicalArtwork.watchMeshExcursion,
                               duration: HUDMechanicalArtwork.watchLoopLegDuration, timingFunction: .linear)
        motion.registerAmbient(layer: artwork.indicatorGlow, key: "glow", keyPath: "opacity",
                               fromValue: -0.12, toValue: 0.12, duration: 1.7)
        actions = labels.enumerated().map { index, label in
            let action = AtelierHudAction()
            action.owner = self; action.index = Int32(index)
            action.setAccessibilityRole(.button); action.setAccessibilityLabel(label)
            action.setAccessibilityParent(self)
            return action
        }
        setAccessibilityChildren(actions)
        map.onCamera = { [weak self] _ in self?.updateMapValue() }
        updateMapValue()
        refreshProfile()
        profileObserver = (try? profileStore.get())?.observe { [weak self] in self?.refreshProfile() }
        accountObserver = account.observe { [weak self] in self?.refreshAccount() }
        refreshAccount()
        accountGauge.onRefresh = { [weak self] in self?.account.refresh(manual: true) }
        setAccessibilityRole(.group)
        setAccessibilityLabel("环形 HUD，原生 EndfieldHUD 地图与导航")
    }
    required init?(coder: NSCoder) { fatalError("Programmatic view") }
    override func updateTrackingAreas() {
        super.updateTrackingAreas()
        if let tracking { removeTrackingArea(tracking) }
        tracking = NSTrackingArea(rect: bounds, options: [.activeInKeyWindow, .mouseMoved, .mouseEnteredAndExited, .inVisibleRect], owner: self)
        addTrackingArea(tracking!)
    }
    override func layout() {
        super.layout()
        peripheralView?.frame = bounds
        scale = min(bounds.width / 1000, bounds.height / 640) * 0.94
        origin = CGPoint(x: (bounds.width - 1000 * scale) / 2, y: (bounds.height - 640 * scale) / 2)
        CATransaction.begin(); CATransaction.setDisableActions(true)
        scene.position = origin
        scene.setAffineTransform(CGAffineTransform(scaleX: scale, y: scale))
        CATransaction.commit()
        for (index, module) in modules.enumerated() {
            if let entry = navigation.entries.first(where: { $0.module == module }), let window {
                let r = entry.interactionRect
                let a = HUDMotionMath.project(CGPoint(x: r.minX - 500, y: r.minY - 320), through: plane.spatial.transform)
                let b = HUDMotionMath.project(CGPoint(x: r.maxX - 500, y: r.maxY - 320), through: plane.spatial.transform)
                let rect = CGRect(x: origin.x + (a.x + 500) * scale, y: origin.y + (a.y + 320) * scale,
                                  width: (b.x - a.x) * scale, height: (b.y - a.y) * scale)
                actions[index].setAccessibilityFrame(window.convertToScreen(convert(rect, to: nil)))
            }
        }
    }
    func updateMapValue() {
        let camera = map.camera
        setAccessibilityValue(String(format: "地图中心 %.2f, %.2f，缩放 %.2f", camera.centerX * 360 - 180, 90 - camera.centerY * 180, camera.zoom))
    }
    override func keyDown(with event: NSEvent) {
        if event.keyCode == 48 {
            // Shift steps backwards one module; plain Tab steps forward.
            focusIndex = (focusIndex + (event.modifierFlags.contains(.shift) ? modules.count - 1 : 1)) % modules.count
            navigation.hover(modules[focusIndex]); return
        }
        if event.keyCode == 36 { onRoute(Int32(focusIndex)); return }
        super.keyDown(with: event)
    }
    func show(_ visible: Bool) {
        isHidden = !visible
        headerClock.setActive(visible)
        account.setVisible(visible, accountModule: false)
        if !visible {
            peripheralView?.interactionEnabled = false
            peripheralView?.stopMotionForConcealment()
            peripheralView?.isHidden = true
            scene.isHidden = false
        }
        if visible { battery.start(); chargeBadge.setStable() } else { battery.stop(); chargeBadge.setStable(visible: false) }
        if visible {
            window?.acceptsMouseMovedEvents = true
            map.prepareForPresentation(); map.activate()
            motion.start(reducedMotion: HUDRuntimeAppearance.reduceMotion)
        } else {
            map.deactivate(); motion.stop(freezePresentation: false); navigation.hover(nil)
        }
    }
    // Same perspective inversion used by the upstream SystemHUDView input host.
    func designPoint(_ event: NSEvent) -> CGPoint {
        let p = convert(event.locationInWindow, from: nil)
        let x = (p.x - origin.x) / scale - 500, y = (p.y - origin.y) / scale - 320
        let t = plane.spatial.presentation()?.transform ?? plane.spatial.transform
        let a = t.m11 - x * t.m14, b = t.m21 - x * t.m24
        let c = t.m12 - y * t.m14, d = t.m22 - y * t.m24
        let u = x * t.m44 - t.m41, v = y * t.m44 - t.m42
        let determinant = a * d - b * c
        return CGPoint(x: (u * d - b * v) / determinant + 500, y: (a * v - u * c) / determinant + 320)
    }
    func mapPoint(_ design: CGPoint) -> CGPoint {
        CGPoint(x: (design.x - mapOrigin.x) / mapScale, y: (design.y - mapOrigin.y) / mapScale)
    }
    override func mouseMoved(with event: NSEvent) {
        let point = designPoint(event)
        identityCard.setHovered(identityCard.target(at: point))
        chargeBadge.setHovered(chargeBadge.contains(point))
        navigation.hoverTarget(navigation.hoverHitTarget(point: point))
        motion.setParallax(normalizedPoint: HUDMotionMath.normalizedPointer(location: point, center: CGPoint(x: 500, y: 320)))
    }
    override func mouseExited(with event: NSEvent) { navigation.hover(nil); motion.resetParallax() }
    override func mouseDown(with event: NSEvent) {
        let p = designPoint(event)
        if let target = identityCard.target(at: p) {
            openPeripheral(target == .profile ? .profile : .power, quit: target == .close)
            return
        }
        if chargeBadge.contains(p) { openPeripheral(.power); return }
        let gaugePoint = CGPoint(x: p.x - header.frame.minX - accountGauge.layer.position.x, y: p.y - header.frame.minY - accountGauge.layer.position.y)
        if CGRect(origin: .zero, size: HUDAccountGauge.size).contains(gaugePoint) { openPeripheral(.account); return }
        if let module = navigation.hitTest(point: p), let index = modules.firstIndex(of: module) {
            navigation.select(module); onRoute(Int32(index)); return
        }
        let point = mapPoint(p)
        if WorldMapSurface.contains(point) { motion.freezeParallax(); map.beginDrag(at: point) }
    }
    override func mouseDragged(with event: NSEvent) {
        if map.isDragging { map.drag(to: mapPoint(designPoint(event))) } else { mouseMoved(with: event) }
    }
    override func mouseUp(with event: NSEvent) { _ = map.endDrag() }
    override func scrollWheel(with event: NSEvent) {
        let point = designPoint(event)
        if navigation.scroll(at: point, delta: event.scrollingDeltaY, phase: event.phase, momentumPhase: event.momentumPhase) { return }
        let p = mapPoint(point)
        if WorldMapSurface.contains(p) { map.zoom(at: p, factor: exp(Double(event.scrollingDeltaY) * 0.015)) }
    }
    override func magnify(with event: NSEvent) {
        let p = mapPoint(designPoint(event))
        if WorldMapSurface.contains(p) { map.zoom(at: p, factor: Double(1 + event.magnification)) }
    }
}

private final class AtelierHudAction: NSAccessibilityElement {
    weak var owner: AtelierHudView?
    var index: Int32 = 0
    override func accessibilityPerformPress() -> Bool { owner?.onRoute(index); return owner != nil }
}

public typealias RouteCallback = @convention(c) (UnsafeMutableRawPointer?, Int32) -> Void
@_cdecl("atelier_hud_create")
public func atelierHudCreate(_ parent: UnsafeMutableRawPointer, _ labelsJSON: UnsafePointer<CChar>,
                      _ callback: RouteCallback, _ context: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let terrain = try? WorldMapTerrain.load(), let countries = try? WorldMapCountries.load() else { return nil }
    let labels = try! JSONDecoder().decode([String].self, from: Data(String(cString: labelsJSON).utf8))
    let view = AtelierHudView(labels: labels, terrain: terrain, countries: countries)
    view.onRoute = { callback(context, $0) }
    let window = Unmanaged<NSWindow>.fromOpaque(parent).takeUnretainedValue()
    window.contentView!.addSubview(view, positioned: .above, relativeTo: nil)
    return Unmanaged.passRetained(view).toOpaque()
}
@_cdecl("atelier_hud_update")
public func atelierHudUpdate(_ pointer: UnsafeMutableRawPointer, _ x: Double, _ y: Double, _ width: Double, _ height: Double, _ visible: Bool) {
    let view = Unmanaged<AtelierHudView>.fromOpaque(pointer).takeUnretainedValue()
    let parent = view.superview!
    // The DOM viewport starts below the titlebar. contentView.bounds may include
    // that titlebar, so use the window's content layout rect as the coordinate origin.
    guard let window = view.window else { return }
    let viewport = window.contentLayoutRect
    let rect = CGRect(x: viewport.minX + x, y: viewport.maxY - y - height,
                      width: width, height: height)
    view.frame = parent.convert(rect, from: nil)
    view.layer?.masksToBounds = true
    view.show(visible)
}
@_cdecl("atelier_hud_destroy")
public func atelierHudDestroy(_ pointer: UnsafeMutableRawPointer) {
    let retained = Unmanaged<AtelierHudView>.fromOpaque(pointer)
    retained.takeUnretainedValue().show(false)
    retained.takeUnretainedValue().removeFromSuperview()
    retained.release()
}
