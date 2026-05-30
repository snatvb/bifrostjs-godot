// ===== Module declaration =====
// `import engine from "bifrostjs"` — единственный импорт для скриптов
// Все типы глобальные, без импорта

declare module "bifrostjs" {
  const engine: Engine
  export default engine
}

// ===== Classes =====

declare class Vector2 {
  constructor(x?: number, y?: number)
  x: number
  y: number
  set(x: number, y: number): void
  add(x: number, y: number): void
}

// ===== Globals (provided by Rust runtime) =====

declare var console: {
  log(...args: any[]): void
  warn(...args: any[]): void
  error(...args: any[]): void
  info(...args: any[]): void
  trace(...args: any[]): void
}

// ===== Script contract =====

interface Script {
  onReady?(): void
  onProcess?(delta: number): void
}

// ===== Engine =====

interface Engine {
  isKeyPressed(key: number): boolean
  createNode(class_name: string): GodotNode | undefined
  instantiate(path: string): GodotNode | undefined
  readonly input: Input
  readonly Keys: Keys
}

interface Input {
  isKeyPressed(key: number): boolean
  readonly x: number
  readonly y: number
}

interface Keys {
  readonly W: 87
  readonly A: 65
  readonly S: 83
  readonly D: 68
}

// ===== Типы-заглушки (без поддержки конвертации) =====

type Color = unknown        // @TODO: Color marshalling
type Rect2 = unknown        // @TODO: Rect2 marshalling
type Vector3 = unknown      // @TODO: Vector3 marshalling
type Transform2D = unknown  // @TODO: Transform2D marshalling
type Transform3D = unknown  // @TODO: Transform3D marshalling

// ===== GodotObjectBase =====
// Базовый интерфейс для любого проксированного Godot-объекта

interface GodotObjectBase {
  readonly gd_instance_id: number
  readonly is_alive: boolean
  readonly name: string
  readonly class_type: string
  readonly id: number
  readonly parent: GodotNode | null

  is_class(class_name: string): boolean
  connect(signal: string, callback: (...args: any[]) => void): number
  disconnect(callback_id: number): void
  register_signal(signal_name: string): void
  emit_signal(signal_name: string, ...args: any[]): void

  // Любое Godot-свойство/метод (runtime proxy)
  [key: string]: any
}

// ===== GodotNodeBase =====
// Методы Node + is_class type guard

interface GodotNodeBase extends GodotObjectBase {
  // lifecycle
  add_child(node: GodotNode, force_readable?: boolean): void
  remove_child(node: GodotNode): void
  queue_free(): void
  duplicate(): GodotNode
  is_inside_tree(): boolean

  // hierarchy
  get_parent(): GodotNode
  get_children(): GodotNode[]
  get_node(path: string): GodotNode
  has_node(path: string): boolean
  get_child_count(): number
  get_index(): number
  move_child(child: GodotNode, idx: number): void
  raise(): void

  // processing
  set_process(enable: boolean): void
  set_physics_process(enable: boolean): void

  // tree
  get_tree(): GodotNode

  // signals
  has_signal(signal: string): boolean
  has_user_signal(signal: string): boolean

  // mouse
  get_global_mouse_position(): Vector2
  get_local_mouse_position(): Vector2

  is_class<T extends string>(name: T): this is GodotNodeByClass<T>
}

// ===== Node2DBase =====

interface Node2DBase extends GodotNodeBase {
  position: Vector2
  global_position: Vector2
  rotation: number
  rotation_degrees: number
  scale: Vector2
  skew: number

  set_position(pos: Vector2): void
  set_global_position(pos: Vector2): void
  set_rotation(rad: number): void
  set_rotation_degrees(deg: number): void
  set_scale(s: Vector2): void
  get_position(): Vector2
  get_global_position(): Vector2
  get_rotation(): number
  get_rotation_degrees(): number
  get_scale(): Vector2
}

// ===== ControlBase =====

interface ControlBase extends GodotNodeBase {
  position: Vector2
  size: Vector2
  custom_minimum_size: Vector2
  rotation: number
  rotation_degrees: number
  scale: Vector2
  pivot_offset: Vector2

  // @TODO: anchor_left, anchor_right, anchor_top, anchor_bottom,
  //        offset_left, offset_right, offset_top, offset_bottom,
  //        grow_horizontal, grow_vertical, size_flags_*
}

// ===== Node3DBase =====

interface Node3DBase extends GodotNodeBase {
  // @TODO: Vector3 — position, rotation, scale, transform, basis, quaternion
  position: Vector3
  rotation: Vector3
  scale: Vector3
}

// ==========================================
//            SPECIFIC TYPES
// ==========================================

// --- Sprite2D ---

interface Sprite2D extends Node2DBase {
  class_type: "Sprite2D"

  texture: GodotTexture | null
  centered: boolean
  offset: Vector2
  flip_h: boolean
  flip_v: boolean
  frame: number
  hframes: number
  vframes: number
  frame_coords: Vector2
  region_enabled: boolean
  region_rect: Rect2 // @TODO
}

// --- AnimatedSprite2D ---

interface AnimatedSprite2D extends Node2DBase {
  class_type: "AnimatedSprite2D"

  animation: string
  autoplay: string
  frame: number
  frame_progress: number
  speed_scale: number
  playing: boolean

  play(name?: string): void
  stop(): void
  pause(): void
  is_playing(): boolean
}

// --- CharacterBody2D ---

interface CharacterBody2D extends Node2DBase {
  class_type: "CharacterBody2D"

  velocity: Vector2

  move_and_slide(): void
  is_on_floor(): boolean
  is_on_ceiling(): boolean
  is_on_wall(): boolean
  get_last_slide_collision(): any // @TODO KinematicCollision2D
  get_slide_collision_count(): number
}

// --- RigidBody2D ---

interface RigidBody2D extends Node2DBase {
  class_type: "RigidBody2D"

  mass: number
  gravity_scale: number
  linear_velocity: Vector2
  angular_velocity: number
  inertia: number
  locked_rotates: boolean
  can_sleep: boolean
  sleeping: boolean
  linear_damp: number
  angular_damp: number
  constant_force: Vector2
  constant_torque: number
  continuous_cd: number // @TODO enum

  apply_central_force(force: Vector2): void
  apply_force(force: Vector2, position: Vector2): void
  apply_impulse(impulse: Vector2, position?: Vector2): void
  apply_torque(torque: number): void
  set_linear_velocity(vel: Vector2): void
  set_angular_velocity(vel: number): void
}

// --- StaticBody2D ---

interface StaticBody2D extends Node2DBase {
  class_type: "StaticBody2D"

  constant_linear_velocity: Vector2
  constant_angular_velocity: number
}

// --- Area2D ---

interface Area2D extends Node2DBase {
  class_type: "Area2D"

  gravity: number
  gravity_direction: Vector2
  gravity_is_point: boolean
  gravity_point_unit_distance: number
  linear_damp: number
  angular_damp: number
  priority: number
  monitorable: boolean
  monitoring: boolean
  // @TODO: collision_mask, collision_layer, area_layer, audio_bus_override

  has_overlapping_bodies(): boolean
  has_overlapping_areas(): boolean
  get_overlapping_bodies(): GodotNode[]
  get_overlapping_areas(): GodotNode[]
  // @TODO: body_entered/body_exited signals via connect
}

// --- CollisionShape2D ---

interface CollisionShape2D extends Node2DBase {
  class_type: "CollisionShape2D"

  disabled: boolean
  one_way_collision: boolean
  one_way_collision_margin: number
  // @TODO: shape — Shape2D resource
}

// --- CollisionPolygon2D ---

interface CollisionPolygon2D extends Node2DBase {
  class_type: "CollisionPolygon2D"

  depth: number
  disabled: boolean
  one_way_collision: boolean
  one_way_collision_margin: number
  polygon: Vector2[] // @TODO PackedVector2Array
}

// --- Camera2D ---

interface Camera2D extends Node2DBase {
  class_type: "Camera2D"

  anchor_mode: number // @TODO enum
  zoom: Vector2
  offset: Vector2
  enabled: boolean
  current: boolean
  limit_smoothed: Vector2
  // @TODO: limit_left, limit_right, limit_top, limit_bottom
  position_smoothing_enabled: boolean
  position_smoothing_speed: number
  drag_horizontal_enabled: boolean
  drag_vertical_enabled: boolean
  drag_margin_h_enabled: boolean
  drag_margin_v_enabled: boolean
  // @TODO: drag margins left/right/top/bottom

  make_current(): void
  clear_current(): void
  is_current(): boolean
}

// --- Timer ---

interface Timer extends GodotNodeBase {
  class_type: "Timer"

  wait_time: number
  one_shot: boolean
  autostart: boolean
  time_left: number

  start(time_sec?: number): void
  stop(): void
  is_stopped(): boolean
  // @TODO: signal timeout
}

// --- AnimationPlayer ---

interface AnimationPlayer extends GodotNodeBase {
  class_type: "AnimationPlayer"

  current_animation: string
  current_assigned_animation: string
  speed_scale: number
  autoplay: string
  playback_default_blend_time: number
  current_animation_length: number
  // @TODO: playback_process_mode (enum)

  play(name?: string): void
  stop(keep_state?: boolean): void
  pause(): void
  seek(seconds: number, update?: boolean): void
  is_playing(): boolean
  get_current_animation(): string
  get_animation(name: string): any // @TODO Animation resource
  clear_queue(): void
}

// --- AudioStreamPlayer2D ---

interface AudioStreamPlayer2D extends Node2DBase {
  class_type: "AudioStreamPlayer2D"

  volume_db: number
  pitch_scale: number
  playing: boolean
  autoplay: boolean
  max_distance: number
  attenuation: number
  panning_strength: number
  // @TODO: stream — AudioStream resource
  // @TODO: bus — StringName

  play(from_position?: number): void
  stop(): void
  is_playing(): boolean
  get_playback_position(): number
}

// --- RayCast2D ---

interface RayCast2D extends Node2DBase {
  class_type: "RayCast2D"

  enabled: boolean
  hit_from_inside: boolean
  target_position: Vector2
  collide_with_bodies: boolean
  collide_with_areas: boolean
  collision_mask: number // @TODO bitmask

  is_colliding(): boolean
  get_collider(): GodotNode | null
  get_collision_normal(): Vector2
  get_collision_point(): Vector2
  get_collision_face_index(): number
  force_raycast_update(): void
  // @TODO: add_exception/add_exception_rid
}

// --- GPUParticles2D ---

interface GPUParticles2D extends Node2DBase {
  class_type: "GPUParticles2D"

  emitting: boolean
  amount: number
  lifetime: number
  one_shot: boolean
  preprocess: number
  speed_scale: number
  explosiveness: number
  randomness: number
  fixed_fps: number
  fract_delta: boolean
  process_material: GodotMaterial | null
  texture: GodotTexture | null
  // @TODO: draw_order, sub_emitter, collision_*

  restart(): void
  set_emitting(emitting: boolean): void
}

// --- CPUParticles2D ---

interface CPUParticles2D extends Node2DBase {
  class_type: "CPUParticles2D"

  emitting: boolean
  amount: number
  lifetime: number
  one_shot: boolean
  preprocess: number
  speed_scale: number
  explosiveness: number
  randomness: number
  fixed_fps: number
  fract_delta: boolean
  texture: GodotTexture | null
  // @TODO: color, color_*, scale_*, angle, gravity

  restart(): void
  set_emitting(emitting: boolean): void
}

// --- Node2D (generic) ---

interface Node2D extends Node2DBase {
  class_type: "Node2D"
  // всё от Node2DBase + прокси
}

// --- Marker2D ---

interface Marker2D extends Node2DBase {
  class_type: "Marker2D"
  gizmo_extents: number
}

// --- Path2D ---

interface Path2D extends Node2DBase {
  class_type: "Path2D"
  curve: any // @TODO Curve2D resource
}

// --- PathFollow2D ---

interface PathFollow2D extends Node2DBase {
  class_type: "PathFollow2D"

  progress: number
  progress_ratio: number
  h_offset: number
  v_offset: number
  rotation_enabled: boolean
  cubic_interp: boolean
  loop: boolean
}

// --- ParallaxBackground ---

interface ParallaxBackground extends Node2DBase {
  class_type: "ParallaxBackground"

  scroll_offset: Vector2
  scroll_scale: Vector2
  scroll_base_offset: Vector2
  scroll_base_scale: Vector2
  scroll_limit_begin: Vector2
  scroll_limit_end: Vector2
}

// --- ParallaxLayer ---

interface ParallaxLayer extends Node2DBase {
  class_type: "ParallaxLayer"

  motion_offset: Vector2
  motion_scale: Vector2
  // @TODO: mirroring
}

// --- TileMap ---

interface TileMap extends Node2DBase {
  class_type: "TileMap"

  // @TODO: tileset — TileSet resource
  rendering_quadrant_size: number
  collision_animation_enabled: boolean
  cell_quadrant_size: number

  set_cell(layer: number, coords: Vector2, source_id: number, atlas_coords?: Vector2, alternative_tile?: number): void
  get_cell_source_id(layer: number, coords: Vector2): number
  get_cell_atlas_coords(layer: number, coords: Vector2): Vector2
  clear_layer(layer: number): void
  clear(): void
  get_used_cells(layer: number): Vector2[]
}

// --- TileMapLayer ---

interface TileMapLayer extends Node2DBase {
  class_type: "TileMapLayer"

  // @TODO: tileset
  enabled: boolean

  set_cell(coords: Vector2, source_id: number, atlas_coords?: Vector2, alternative_tile?: number): void
  get_cell_source_id(coords: Vector2): number
  clear(): void
}

// --- VisibleOnScreenNotifier2D ---

interface VisibleOnScreenNotifier2D extends Node2DBase {
  class_type: "VisibleOnScreenNotifier2D"

  rect: Rect2 // @TODO

  is_on_screen(): boolean
}

// --- VisibleOnScreenEnabler2D ---

interface VisibleOnScreenEnabler2D extends Node2DBase {
  class_type: "VisibleOnScreenEnabler2D"

  enable_mode: number // @TODO enum
  rect: Rect2 // @TODO
}

// --- MeshInstance2D ---

interface MeshInstance2D extends Node2DBase {
  class_type: "MeshInstance2D"
  // @TODO: mesh — Mesh resource
  texture: GodotTexture | null
}

// --- NavigationAgent2D ---

interface NavigationAgent2D extends Node2DBase {
  class_type: "NavigationAgent2D"

  target_position: Vector2
  navigation_layers: number // @TODO bitmask
  path_desired_distance: number
  target_desired_distance: number
  path_max_distance: number
  velocity: Vector2
  max_speed: number

  is_navigation_finished(): boolean
  get_next_path_position(): Vector2
  get_current_navigation_result(): any // @TODO
  // @TODO: velocity computation mode
}

// --- NavigationObstacle2D ---

interface NavigationObstacle2D extends Node2DBase {
  class_type: "NavigationObstacle2D"

  radius: number
  velocity: Vector2
  // @TODO: avoidance_layers, height
}

// --- NavigationRegion2D ---

interface NavigationRegion2D extends Node2DBase {
  class_type: "NavigationRegion2D"

  enabled: boolean
  // @TODO: navigation_polygon — NavigationPolygon resource
  // @TODO: layers
}

// --- CanvasLayer ---

interface CanvasLayer extends GodotNodeBase {
  class_type: "CanvasLayer"

  layer: number
  follow_viewport_enabled: boolean
  follow_viewport_scale: number
  rotation: number
  scale: Vector2
  offset: Vector2
  // @TODO: visible, modulate, custom_viewport
}

// ==========================================
//            CONTROL TYPES
// ==========================================

// --- Control ---

interface Control extends ControlBase {
  visible: boolean
  mouse_filter: number // @TODO enum
  mouse_force_pass: boolean
  // @TODO: theme, theme_type_variation

  show(): void
  hide(): void
  get_parent_area_size(): Vector2
  get_minimum_size(): Vector2
  set_anchor_and_offset(...args: any[]): void
}

// --- Label ---

interface Label extends Control {
  class_type: "Label"

  text: string
  horizontal_alignment: number // @TODO enum
  vertical_alignment: number // @TODO enum
  autowrap_mode: number // @TODO enum
  clip_text: boolean
  max_lines_visible: number
  language: string
  // @TODO: label_settings — LabelSettings
  // @TODO: ellipsize_behavior

  get_line_count(): number
  get_visible_line_count(): number
}

// --- Button ---

interface Button extends Omit<Control, 'class_type'> {
  class_type: "Button"

  text: string
  disabled: boolean
  flat: boolean
  icon: GodotTexture | null
  icon_alignment: number // @TODO enum
  // @TODO: button_group, button_mask, shortcut
}

// --- TextureButton ---

interface TextureButton extends Control {
  class_type: "TextureButton"

  texture_normal: GodotTexture | null
  texture_pressed: GodotTexture | null
  texture_hover: GodotTexture | null
  texture_disabled: GodotTexture | null
  texture_focused: GodotTexture | null
  expand: boolean
  stretch_mode: number // @TODO enum
  flip_h: boolean
  flip_v: boolean
  ignore_texture_size: boolean
  // @TODO: click_mask — BitMap
}

// --- TextureRect ---

interface TextureRect extends Control {
  class_type: "TextureRect"

  texture: GodotTexture | null
  expand_mode: number // @TODO enum
  stretch_mode: number // @TODO enum
  flip_h: boolean
  flip_v: boolean
}

// --- ColorRect ---

interface ColorRect extends Control {
  class_type: "ColorRect"

  color: Color // @TODO
}

// --- RichTextLabel ---

interface RichTextLabel extends Control {
  class_type: "RichTextLabel"

  text: string
  bbcode_enabled: boolean
  bbcode_text: string
  // @TODO: label_settings, mouse_filter, autowrap_mode, scroll_active

  append_text(text: string): void
  clear(): void
  get_parsed_bbcode(): string
  get_content_height(): number
  get_content_width(): number
  get_line_count(): number
  get_visible_line_count(): number
  scroll_to_line(line: number): void
}

// --- LineEdit ---

interface LineEdit extends Control {
  class_type: "LineEdit"

  text: string
  placeholder_text: string
  placeholder_alignment: number // @TODO enum
  editable: boolean
  max_length: number
  caret_column: number
  readonly: boolean
  select_all_on_focus: boolean
  // @TODO: caret_blink, caret_force_displayed, middle_mouse_paste

  select(from?: number, to?: number): void
  select_all(): void
  deselect(): void
  get_text(): string
  clear(): void
  cut_text(): string
  copy_text(): string
  paste_text(): void
  has_selection(): boolean
}

// --- OptionButton ---

interface OptionButton extends Omit<Button, 'class_type'> {
  class_type: "OptionButton"

  selected: number
  // @TODO: item_count, fit_to_longest_item, align

  get_item_text(index: number): string
  select(index: number): void
  add_item(text: string, icon?: GodotTexture): void
  add_separator(): void
  clear(): void
  get_popup(): any // @TODO PopupMenu
}

// --- CheckButton / CheckBox ---

interface CheckButton extends Omit<Button, 'class_type'> {
  class_type: "CheckButton"
  // button_mask, pressed — через Button
}

interface CheckBox extends Omit<Button, 'class_type'> {
  class_type: "CheckBox"
}

// --- SpinBox ---

interface SpinBox extends Control {
  class_type: "SpinBox"

  value: number
  min_value: number
  max_value: number
  step: number
  prefix: string
  suffix: string
  editable: boolean
  // @TODO: alignment, custom_arrow_step
}

// --- HSlider / VSlider ---

interface HSlider extends Control {
  class_type: "HSlider"

  value: number
  min_value: number
  max_value: number
  step: number
  tick_count: number
  // @TODO: editable, scrollable, rounded
}

interface VSlider extends Control {
  class_type: "VSlider"

  value: number
  min_value: number
  max_value: number
  step: number
  tick_count: number
}

// --- ProgressBar ---

interface ProgressBar extends Control {
  class_type: "ProgressBar"

  value: number
  min_value: number
  max_value: number
  show_percentage: boolean
  // @TODO: fill_mode, nine_patch_stretch, orientation
}

// --- LinkButton ---

interface LinkButton extends Omit<Button, 'class_type'> {
  class_type: "LinkButton"

  uri: string
}

// --- PopupMenu ---

interface PopupMenu extends Control {
  class_type: "PopupMenu"

  // @TODO: items, hide_on_checkable_item_selection, hide_on_item_selection
  show(): void
  hide(): void
  add_item(text: string, id?: number): void
  add_check_item(text: string, id?: number): void
  add_separator(): void
  clear(): void
  set_item_text(index: number, text: string): void
  get_item_text(index: number): string
  is_item_checkable(index: number): boolean
  set_item_as_checkable(index: number, checkable: boolean): void
  is_item_checked(index: number): boolean
  set_item_checked(index: number, checked: boolean): void
}

// --- ItemList ---

interface ItemList extends Control {
  class_type: "ItemList"

  max_columns: number
  same_column_width: boolean
  allow_reselect: boolean
  auto_height: boolean
  // @TODO: icon_mode, icon_scale, select_mode, fixed_column_width

  add_item(text: string, icon?: GodotTexture): void
  add_icon_item(icon: GodotTexture): void
  clear(): void
  select(index: number, single?: boolean): void
  deselect(index: number): void
  deselect_all(): void
  get_selected_items(): number[]
  is_selected(index: number): boolean
  get_item_text(index: number): string
  set_item_text(index: number, text: string): void
  get_item_count(): number
}

// --- Tree ---

interface Tree extends Control {
  class_type: "Tree"

  columns: number
  hide_root: boolean
  // @TODO: huge API — create_item, get_next, etc.
  clear(): void
  get_root(): any // @TODO TreeItem
  create_item(parent?: any): any // @TODO TreeItem
  get_selected(): any // @TODO TreeItem
  // @TODO: scroll_to_item, ensure_cursor_is_visible
}

// --- TabContainer ---

interface TabContainer extends Control {
  class_type: "TabContainer"

  current_tab: number
  tabs_count: number
  // @TODO: tab_alignment, tab_layout
  // @TODO: drag_to_rearrange_enabled, tabs_visible
}

// --- Window ---

interface Window extends Control {
  class_type: "Window"

  title: string
  size: Vector2
  visible: boolean
  // @TODO: mode, keep_border, borderless, unresizable
  // @TODO: exclusive, transient, popup_window
  show(): void
  hide(): void
  close_request(): void
}

// --- Container layout types ---

interface Container extends Omit<Control, 'class_type'> {
  class_type: "Container"
  // @TODO: separate layout methods per container type
}

interface Panel extends Omit<Container, 'class_type'> {
  class_type: "Panel"
}

interface PanelContainer extends Omit<Container, 'class_type'> {
  class_type: "PanelContainer"
}

interface VBoxContainer extends Omit<Container, 'class_type'> {
  class_type: "VBoxContainer"
}

interface HBoxContainer extends Omit<Container, 'class_type'> {
  class_type: "HBoxContainer"
}

interface GridContainer extends Omit<Container, 'class_type'> {
  class_type: "GridContainer"
  columns: number
}

interface MarginContainer extends Omit<Container, 'class_type'> {
  class_type: "MarginContainer"
  // @TODO: theme_override/constants
}

interface CenterContainer extends Omit<Container, 'class_type'> {
  class_type: "CenterContainer"
  use_top_level: boolean
}

interface AspectRatioContainer extends Omit<Container, 'class_type'> {
  class_type: "AspectRatioContainer"
  ratio: number
  stretch_mode: number // @TODO enum
  alignment_h: number // @TODO enum
  alignment_v: number // @TODO enum
}

interface ScrollContainer extends Omit<Container, 'class_type'> {
  class_type: "ScrollContainer"

  scroll_horizontal: number
  scroll_vertical: number
  scroll_horizontal_enabled: boolean
  scroll_vertical_enabled: boolean
  // @TODO: deadzone, follow_focus
}

interface SplitContainer extends Omit<Container, 'class_type'> {
  class_type: "SplitContainer" | "HSplitContainer" | "VSplitContainer"

  split_offset: number
  dragger_visibility: number // @TODO enum
  collapsed: boolean
  vertical: boolean
}

// --- HSplitContainer / VSplitContainer ---

interface HSplitContainer extends Omit<SplitContainer, 'class_type'> {
  class_type: "HSplitContainer"
}

interface VSplitContainer extends Omit<SplitContainer, 'class_type'> {
  class_type: "VSplitContainer"
}

// --- MenuBar ---

interface MenuBar extends Control {
  class_type: "MenuBar"

  // @TODO: provide_menu, menu_items
}

// ==========================================
//           3D TYPES
// ==========================================

interface Node3D extends Node3DBase {
  class_type: "Node3D"
}

interface CharacterBody3D extends Node3DBase {
  class_type: "CharacterBody3D"
  velocity: Vector3
  // @TODO: move_and_slide, is_on_floor etc (Vector3 variant)
}

interface RigidBody3D extends Node3DBase {
  class_type: "RigidBody3D"
  mass: number
  gravity_scale: number
  linear_velocity: Vector3
  angular_velocity: Vector3
}

interface StaticBody3D extends Node3DBase {
  class_type: "StaticBody3D"
}

interface Area3D extends Node3DBase {
  class_type: "Area3D"
  gravity: number
  linear_damp: number
  angular_damp: number
  monitoring: boolean
  monitorable: boolean
}

interface CollisionShape3D extends Node3DBase {
  class_type: "CollisionShape3D"
  disabled: boolean
}

interface CollisionPolygon3D extends Node3DBase {
  class_type: "CollisionPolygon3D"
  disabled: boolean
}

interface Camera3D extends Node3DBase {
  class_type: "Camera3D"
  current: boolean
  // @TODO: fov, near, far, h_offset, v_offset
}

interface Path3D extends Node3DBase {
  class_type: "Path3D"
  curve: any // @TODO Curve3D
}

interface PathFollow3D extends Node3DBase {
  class_type: "PathFollow3D"
  progress: number
  progress_ratio: number
  rotation_enabled: boolean
}

interface MeshInstance3D extends Node3DBase {
  class_type: "MeshInstance3D"
  // @TODO: mesh, material_override
}

interface AudioStreamPlayer3D extends Node3DBase {
  class_type: "AudioStreamPlayer3D"
  volume_db: number
  playing: boolean
  play(): void
  stop(): void
}

interface RayCast3D extends Node3DBase {
  class_type: "RayCast3D"
  enabled: boolean
  target_position: Vector3
  is_colliding(): boolean
  get_collider(): GodotNode | null
}

interface VisibilityNotifier3D extends Node3DBase {
  class_type: "VisibleOnScreenNotifier3D" | "VisibilityNotifier3D"
  is_on_screen(): boolean
}

interface GPUParticles3D extends Node3DBase {
  class_type: "GPUParticles3D"
  emitting: boolean
  amount: number
  lifetime: number
  restart(): void
}

interface CPUParticles3D extends Node3DBase {
  class_type: "CPUParticles3D"
  emitting: boolean
  amount: number
  lifetime: number
  restart(): void
}

interface NavigationAgent3D extends Node3DBase {
  class_type: "NavigationAgent3D"
  target_position: Vector3
  max_speed: number
  velocity: Vector3
  is_navigation_finished(): boolean
}

interface NavigationObstacle3D extends Node3DBase {
  class_type: "NavigationObstacle3D"
  radius: number
  velocity: Vector3
}

interface NavigationRegion3D extends Node3DBase {
  class_type: "NavigationRegion3D"
  enabled: boolean
}

interface BoneAttachment3D extends Node3DBase {
  class_type: "BoneAttachment3D"
}

interface Marker3D extends Node3DBase {
  class_type: "Marker3D"
}

interface SpringArm3D extends Node3DBase {
  class_type: "SpringArm3D"
  spring_length: number
  collision_mask: number
}

// ==========================================
//          GodotNode — union
// ==========================================

type GodotNode =
  // Node2D hierarchy
  | Node2D
  | Sprite2D
  | AnimatedSprite2D
  | CharacterBody2D
  | RigidBody2D
  | StaticBody2D
  | Area2D
  | CollisionShape2D
  | CollisionPolygon2D
  | Camera2D
  | Path2D
  | PathFollow2D
  | ParallaxBackground
  | ParallaxLayer
  | TileMap
  | TileMapLayer
  | Marker2D
  | VisibleOnScreenNotifier2D
  | VisibleOnScreenEnabler2D
  | GPUParticles2D
  | CPUParticles2D
  | RayCast2D
  | NavigationAgent2D
  | NavigationObstacle2D
  | NavigationRegion2D
  | MeshInstance2D
  // Control hierarchy
  | Control
  | Label
  | Button
  | TextureButton
  | TextureRect
  | ColorRect
  | RichTextLabel
  | LineEdit
  | OptionButton
  | CheckButton
  | CheckBox
  | SpinBox
  | HSlider
  | VSlider
  | ProgressBar
  | LinkButton
  | PopupMenu
  | ItemList
  | Tree
  | TabContainer
  | Window
  | Container
  | Panel
  | PanelContainer
  | VBoxContainer
  | HBoxContainer
  | GridContainer
  | MarginContainer
  | CenterContainer
  | AspectRatioContainer
  | ScrollContainer
  | HSplitContainer
  | VSplitContainer
  | MenuBar
  // Node (non-CanvasItem)
  | Timer
  | AnimationPlayer
  | CanvasLayer
  | AudioStreamPlayer2D
  // Node3D hierarchy
  | Node3D
  | CharacterBody3D
  | RigidBody3D
  | StaticBody3D
  | Area3D
  | CollisionShape3D
  | CollisionPolygon3D
  | Camera3D
  | Path3D
  | PathFollow3D
  | MeshInstance3D
  | AudioStreamPlayer3D
  | RayCast3D
  | VisibilityNotifier3D
  | GPUParticles3D
  | CPUParticles3D
  | NavigationAgent3D
  | NavigationObstacle3D
  | NavigationRegion3D
  | BoneAttachment3D
  | Marker3D
  | SpringArm3D

// ==========================================
//          GodotNodeByClass conditional
// ==========================================

type GodotNodeByClass<T extends string> =
  T extends "Node" | "CanvasItem" ? GodotNodeBase :
  T extends "Node2D" ? Node2D :
  T extends "Sprite2D" ? Sprite2D :
  T extends "AnimatedSprite2D" ? AnimatedSprite2D :
  T extends "CharacterBody2D" ? CharacterBody2D :
  T extends "RigidBody2D" ? RigidBody2D :
  T extends "StaticBody2D" ? StaticBody2D :
  T extends "Area2D" ? Area2D :
  T extends "CollisionShape2D" ? CollisionShape2D :
  T extends "CollisionPolygon2D" ? CollisionPolygon2D :
  T extends "Camera2D" ? Camera2D :
  T extends "Path2D" ? Path2D :
  T extends "PathFollow2D" ? PathFollow2D :
  T extends "ParallaxBackground" ? ParallaxBackground :
  T extends "ParallaxLayer" ? ParallaxLayer :
  T extends "TileMap" ? TileMap :
  T extends "TileMapLayer" ? TileMapLayer :
  T extends "Marker2D" ? Marker2D :
  T extends "VisibleOnScreenNotifier2D" ? VisibleOnScreenNotifier2D :
  T extends "VisibleOnScreenEnabler2D" ? VisibleOnScreenEnabler2D :
  T extends "GPUParticles2D" ? GPUParticles2D :
  T extends "CPUParticles2D" ? CPUParticles2D :
  T extends "RayCast2D" ? RayCast2D :
  T extends "NavigationAgent2D" ? NavigationAgent2D :
  T extends "NavigationObstacle2D" ? NavigationObstacle2D :
  T extends "NavigationRegion2D" ? NavigationRegion2D :
  T extends "MeshInstance2D" ? MeshInstance2D :
  T extends "Control" ? Control :
  T extends "Label" ? Label :
  T extends "Button" ? Button :
  T extends "TextureButton" ? TextureButton :
  T extends "TextureRect" ? TextureRect :
  T extends "ColorRect" ? ColorRect :
  T extends "RichTextLabel" ? RichTextLabel :
  T extends "LineEdit" ? LineEdit :
  T extends "OptionButton" ? OptionButton :
  T extends "CheckButton" ? CheckButton :
  T extends "CheckBox" ? CheckBox :
  T extends "SpinBox" ? SpinBox :
  T extends "HSlider" ? HSlider :
  T extends "VSlider" ? VSlider :
  T extends "ProgressBar" ? ProgressBar :
  T extends "LinkButton" ? LinkButton :
  T extends "PopupMenu" ? PopupMenu :
  T extends "ItemList" ? ItemList :
  T extends "Tree" ? Tree :
  T extends "TabContainer" ? TabContainer :
  T extends "Window" ? Window :
  T extends "Container" ? Container :
  T extends "Panel" ? Panel :
  T extends "PanelContainer" ? PanelContainer :
  T extends "VBoxContainer" ? VBoxContainer :
  T extends "HBoxContainer" ? HBoxContainer :
  T extends "GridContainer" ? GridContainer :
  T extends "MarginContainer" ? MarginContainer :
  T extends "CenterContainer" ? CenterContainer :
  T extends "AspectRatioContainer" ? AspectRatioContainer :
  T extends "ScrollContainer" ? ScrollContainer :
  T extends "HSplitContainer" ? HSplitContainer :
  T extends "VSplitContainer" ? VSplitContainer :
  T extends "MenuBar" ? MenuBar :
  T extends "Timer" ? Timer :
  T extends "AnimationPlayer" ? AnimationPlayer :
  T extends "CanvasLayer" ? CanvasLayer :
  T extends "AudioStreamPlayer2D" ? AudioStreamPlayer2D :
  T extends "Node3D" ? Node3D :
  T extends "CharacterBody3D" ? CharacterBody3D :
  T extends "RigidBody3D" ? RigidBody3D :
  T extends "StaticBody3D" ? StaticBody3D :
  T extends "Area3D" ? Area3D :
  T extends "CollisionShape3D" ? CollisionShape3D :
  T extends "CollisionPolygon3D" ? CollisionPolygon3D :
  T extends "Camera3D" ? Camera3D :
  T extends "Path3D" ? Path3D :
  T extends "PathFollow3D" ? PathFollow3D :
  T extends "MeshInstance3D" ? MeshInstance3D :
  T extends "AudioStreamPlayer3D" ? AudioStreamPlayer3D :
  T extends "RayCast3D" ? RayCast3D :
  T extends "VisibleOnScreenNotifier3D" | "VisibilityNotifier3D" ? VisibilityNotifier3D :
  T extends "GPUParticles3D" ? GPUParticles3D :
  T extends "CPUParticles3D" ? CPUParticles3D :
  T extends "NavigationAgent3D" ? NavigationAgent3D :
  T extends "NavigationObstacle3D" ? NavigationObstacle3D :
  T extends "NavigationRegion3D" ? NavigationRegion3D :
  T extends "BoneAttachment3D" ? BoneAttachment3D :
  T extends "Marker3D" ? Marker3D :
  T extends "SpringArm3D" ? SpringArm3D :
  GodotNodeBase // fallback

// ==========================================
//        RESOURCE / TEXTURE / MATERIAL
// ==========================================

interface GodotResource extends GodotObjectBase {
  readonly class_type: ResourceClassType
  readonly resource_path: string
  readonly resource_name: string
  duplicate(): GodotResource
  take_over_path(path: string): void
}

interface GodotTexture extends GodotObjectBase {
  readonly class_type: TextureClassType
  readonly size: Vector2
  // @TODO: get_width, get_height, get_image, create_from_image
}

interface GodotMaterial extends GodotObjectBase {
  readonly class_type: MaterialClassType
  // @TODO: shader_param/set_shader_param, next_pass etc
}

// ==========================================
//          GodotObject union
// ==========================================

type GodotObject = GodotNode | GodotResource | GodotTexture | GodotMaterial

// ==========================================
//    Type literals (for class_type narrowing)
// ==========================================

type NodeClassType =
  | "Node"
  | "CanvasItem"
  | "Node2D"

type ControlClassType =
  | "Control"
  | "Container"
  | "Panel"
  | "PanelContainer"
  | "Button"
  | "Label"
  | "TextureRect"
  | "ColorRect"
  | "LineEdit"
  | "RichTextLabel"
  | "ScrollContainer"
  | "VBoxContainer"
  | "HBoxContainer"
  | "GridContainer"
  | "MarginContainer"
  | "AspectRatioContainer"
  | "CenterContainer"
  | "TabContainer"
  | "HSplitContainer"
  | "VSplitContainer"
  | "Tree"
  | "ItemList"
  | "OptionButton"
  | "CheckButton"
  | "CheckBox"
  | "SpinBox"
  | "HSlider"
  | "VSlider"
  | "ProgressBar"
  | "TextureButton"
  | "LinkButton"
  | "MenuBar"
  | "PopupMenu"
  | "Window"

type Node2DClassType =
  | "Sprite2D"
  | "AnimatedSprite2D"
  | "CharacterBody2D"
  | "RigidBody2D"
  | "StaticBody2D"
  | "CharacterBody3D"
  | "RigidBody3D"
  | "StaticBody3D"
  | "Area2D"
  | "Area3D"
  | "CollisionShape2D"
  | "CollisionPolygon2D"
  | "CollisionShape3D"
  | "CollisionPolygon3D"
  | "Camera2D"
  | "Camera3D"
  | "Path2D"
  | "Path3D"
  | "PathFollow2D"
  | "PathFollow3D"
  | "ParallaxBackground"
  | "ParallaxLayer"
  | "TileMap"
  | "TileMapLayer"
  | "Node2D"
  | "Marker2D"
  | "VisibleOnScreenNotifier2D"
  | "VisibleOnScreenEnabler2D"
  | "GPUParticles2D"
  | "CPUParticles2D"
  | "RayCast2D"
  | "RayCast3D"
  | "NavigationAgent2D"
  | "NavigationAgent3D"
  | "NavigationObstacle2D"
  | "NavigationObstacle3D"
  | "NavigationRegion2D"
  | "NavigationRegion3D"
  | "MeshInstance2D"
  | "MeshInstance3D"

type OtherNodeClassType =
  | "AnimationPlayer"
  | "AudioStreamPlayer2D"
  | "AudioStreamPlayer3D"
  | "Timer"
  | "GPUParticles3D"
  | "CPUParticles3D"
  | "VisibleOnScreenNotifier3D"
  | "CanvasLayer"
  | "Node3D"
  | "BoneAttachment3D"
  | "Marker3D"
  | "SpringArm3D"

type ResourceClassType =
  | "Resource"
  | "PackedScene"
  | "Animation"
  | "Shader"
  | "StyleBox"
  | "Theme"
  | "Curve"
  | "Curve2D"
  | "Curve3D"
  | "Gradient"
  | "Mesh"
  | "ArrayMesh"
  | "NavigationMesh"
  | "Noise"
  | "FastNoiseLite"

type TextureClassType =
  | "Texture2D"
  | "ImageTexture"
  | "CompressedTexture2D"
  | "ViewportTexture"
  | "AtlasTexture"
  | "GradientTexture2D"
  | "NoiseTexture2D"
  | "CurveTexture"
  | "CameraTexture"
  | "Texture3D"
  | "CompressedTexture3D"
  | "PlaceholderTexture2D"
  | "TextureLayered"

type MaterialClassType =
  | "Material"
  | "ShaderMaterial"
  | "StandardMaterial3D"
  | "CanvasItemMaterial"
  | "ParticleProcessMaterial"
  | "ORMMaterial3D"
  | "BaseMaterial3D"
