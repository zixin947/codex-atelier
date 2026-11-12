import AppKit
import QuartzCore

// AppKit host only. Rendering, hover animation, camera and geographic painting
// remain in the vendored EndfieldHUD classes.
private final class AtelierHudView: NSView {
    override var isFlipped: Bool { true }
    let navigation: HUDNavigation
    let map: WorldMapSurface
    let artwork = HUDMechanicalArtwork()
    let plane = HUDDepthPlane(name: "atelier", depth: 0, travel: 8, lag: 0.28)
    let scene = CALayer()
    lazy var motion = HUDMotionController(planes: [plane])
    let modules: [HUDModule] = [.system, .display, .hotkeys, .about, .storage, .activityMonitor,
                                .notes, .fileShelf, .clipboard, .archive, .mediaAssembly, .reader, .calendar, .map]
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
            focusIndex = (focusIndex + (event.modifierFlags.contains(.shift) ? 13 : 1)) % modules.count
            navigation.hover(modules[focusIndex]); return
        }
        if event.keyCode == 36 { onRoute(Int32(focusIndex)); return }
        super.keyDown(with: event)
    }
    func show(_ visible: Bool) {
        isHidden = !visible
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
        navigation.hoverTarget(navigation.hoverHitTarget(point: point))
        motion.setParallax(normalizedPoint: HUDMotionMath.normalizedPointer(location: point, center: CGPoint(x: 500, y: 320)))
    }
    override func mouseExited(with event: NSEvent) { navigation.hover(nil); motion.resetParallax() }
    override func mouseDown(with event: NSEvent) {
        let p = designPoint(event)
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
    view.frame = CGRect(x: x, y: view.superview!.bounds.height - y - height, width: width, height: height)
    view.show(visible)
}
@_cdecl("atelier_hud_destroy")
public func atelierHudDestroy(_ pointer: UnsafeMutableRawPointer) {
    let retained = Unmanaged<AtelierHudView>.fromOpaque(pointer)
    retained.takeUnretainedValue().show(false)
    retained.takeUnretainedValue().removeFromSuperview()
    retained.release()
}
