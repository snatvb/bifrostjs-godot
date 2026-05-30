// AI Generated file

declare module "bifrostjs" {
  const engine: Engine
  export default engine
}

// ===== Globals (provided by Rust runtime) =====

declare var console: {
  log(...args: any[]): void
  warn(...args: any[]): void
  error(...args: any[]): void
  info(...args: any[]): void
  trace(...args: any[]): void
}

// ===== Classes =====

declare class Vector2 {
  constructor(x?: number, y?: number)
  x: number
  y: number
  set(x: number, y: number): void
  add(x: number, y: number): void
}

declare class Color {
  constructor(r: number, g: number, b: number, a: number)
  r: number
  g: number
  b: number
  a: number
  set(r: number, g: number, b: number, a: number): void
  toRgba32(): number
  toString(): string
}

// ===== Script contract =====

interface Script {
  onReady?(): void
  onProcess?(delta: number): void
}

// ===== Engine =====

interface Engine {
  isKeyPressed(key: number): boolean
  createNode<T extends string>(class_name: T): GodotNodeByClass<T> | undefined
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
  // letters
  readonly A: 65
  readonly B: 66
  readonly C: 67
  readonly D: 68
  readonly E: 69
  readonly F: 70
  readonly G: 71
  readonly H: 72
  readonly I: 73
  readonly J: 74
  readonly K: 75
  readonly L: 76
  readonly M: 77
  readonly N: 78
  readonly O: 79
  readonly P: 80
  readonly Q: 81
  readonly R: 82
  readonly S: 83
  readonly T: 84
  readonly U: 85
  readonly V: 86
  readonly W: 87
  readonly X: 88
  readonly Y: 89
  readonly Z: 90

  // digits
  readonly KEY_0: 48
  readonly KEY_1: 49
  readonly KEY_2: 50
  readonly KEY_3: 51
  readonly KEY_4: 52
  readonly KEY_5: 53
  readonly KEY_6: 54
  readonly KEY_7: 55
  readonly KEY_8: 56
  readonly KEY_9: 57

  // arrows
  readonly UP: 4194320
  readonly DOWN: 4194321
  readonly LEFT: 4194319
  readonly RIGHT: 4194322

  // modifiers
  readonly SHIFT: 4194305
  readonly CTRL: 4194307
  readonly ALT: 4194309
  readonly META: 4194306

  // navigation
  readonly SPACE: 32
  readonly ENTER: 4194308
  readonly ESCAPE: 4194303
  readonly TAB: 4194302
  readonly BACKSPACE: 4194301
  readonly DELETE: 4194311
  readonly HOME: 4194318
  readonly END: 4194323
  readonly PAGEUP: 4194316
  readonly PAGEDOWN: 4194317
  readonly INSERT: 4194315

  // function keys
  readonly F1: 4194324
  readonly F2: 4194325
  readonly F3: 4194326
  readonly F4: 4194327
  readonly F5: 4194328
  readonly F6: 4194329
  readonly F7: 4194330
  readonly F8: 4194331
  readonly F9: 4194332
  readonly F10: 4194333
  readonly F11: 4194334
  readonly F12: 4194335
}

// ===== Маршаллинг типы =====

declare class Rect2 {
  constructor(x?: number, y?: number, width?: number, height?: number)
  x: number
  y: number
  width: number
  height: number
  set(x: number, y: number, width: number, height: number): void
}

// ===== Маршаллинг типы =====

type Vector3 = unknown      // @TODO: Vector3 marshalling
type Transform3D = unknown  // @TODO: Transform3D marshalling

declare class Transform2D {
  constructor(xx: number, xy: number, yx: number, yy: number, ox: number, oy: number)
  xx: number
  xy: number
  yx: number
  yy: number
  ox: number
  oy: number
  set_origin(x: number, y: number): void
}

// ===== GodotObjectBase =====

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
}

// ===== GodotNodeBase =====

interface GodotNodeBase extends GodotObjectBase {
  // lifecycle
  add_child(node: GodotNode, force_readable?: boolean): void
  remove_child(node: GodotNode): void
  queue_free(): void
  duplicate(): GodotNode
  is_inside_tree(): boolean

  // hierarchy
  get_parent(): GodotNode | null
  get_children(): GodotNode[]
  get_child(idx: number, include_internal?: boolean): GodotNode
  get_node<T extends GodotNode = GodotNode>(path: string): T
  get_node_or_null<T extends GodotNode = GodotNode>(path: string): T | null
  has_node(path: string): boolean
  get_child_count(): number
  get_index(): number
  move_child(child: GodotNode, idx: number): void
  raise(): void
  add_sibling(sibling: GodotNode, force_readable?: boolean): void
  reparent(new_parent: GodotNode, keep_global_transform?: boolean): void
  replace_by(node: GodotNode, keep_groups?: boolean): void

  // owner / scene
  owner: GodotNode | null
  scene_file_path: string
  unique_name_in_owner: boolean

  // processing
  process_mode: number
  process_priority: number
  process_physics_priority: number
  physics_interpolation_mode: number

  set_process(enable: boolean): void
  set_physics_process(enable: boolean): void
  set_process_input(enable: boolean): void
  set_process_unhandled_input(enable: boolean): void
  set_process_shortcut_input(enable: boolean): void
  is_processing(): boolean
  is_physics_processing(): boolean
  can_process(): boolean
  is_node_ready(): boolean
  request_ready(): void

  // tree
  get_tree(): GodotNode | null
  get_viewport(): GodotNode | null
  get_window(): GodotNode | null
  is_ancestor_of(node: GodotNode): boolean
  is_inside_tree(): boolean

  // search
  find_child(pattern: string, recursive?: boolean, owned?: boolean): GodotNode | null
  find_children(pattern: string, type?: string, recursive?: boolean, owned?: boolean): GodotNode[]
  find_parent(pattern: string): GodotNode | null

  // path
  get_path(): string
  get_path_to(node: GodotNode, use_unique_path?: boolean): string

  // groups
  add_to_group(group: string, persistent?: boolean): void
  remove_from_group(group: string): void
  is_in_group(group: string): boolean
  get_groups(): string[]

  // tween
  create_tween(): any

  // signals
  has_signal(signal: string): boolean
  has_user_signal(signal: string): boolean

  // notification / propagation
  propagate_call(method: string, args?: any[], parent_first?: boolean): void
  propagate_notification(what: number): void
  set_editable_instance(node: GodotNode, editable: boolean): void
  is_editable_instance(node: GodotNode): boolean

  // debug
  print_tree(): void
  print_tree_pretty(): void
  get_tree_string(): string
  get_tree_string_pretty(): string

  // mouse
  get_global_mouse_position(): Vector2
  get_local_mouse_position(): Vector2

  is_class<T extends string>(name: T): this is GodotNodeByClass<T>
}

// ===== CanvasItemBase =====

interface CanvasItemBase extends GodotNodeBase {
  visible: boolean
  modulate: Color
  self_modulate: Color
  z_index: number
  z_as_relative: boolean
  material: GodotMaterial | null
  use_parent_material: boolean
  show_behind_parent: boolean
  top_level: boolean
  clip_children: number
  light_mask: number
  visibility_layer: number
  texture_filter: number
  texture_repeat: number

  show(): void
  hide(): void
  is_visible_in_tree(): boolean
  queue_redraw(): void

  set_notify_transform(enable: boolean): void
  set_notify_local_transform(enable: boolean): void
  is_transform_notification_enabled(): boolean
  is_local_transform_notification_enabled(): boolean

  get_canvas_transform(): Transform2D
  get_global_transform(): Transform2D
  get_viewport_rect(): Rect2
  get_viewport(): GodotNode | null

  set_instance_shader_parameter(name: string, value: any): void
}

// ===== Node2DBase =====

interface Node2DBase extends CanvasItemBase {
  position: Vector2
  global_position: Vector2
  rotation: number
  rotation_degrees: number
  scale: Vector2
  skew: number
  global_rotation: number
  global_rotation_degrees: number
  global_scale: Vector2
  global_skew: number
  transform: Transform2D
  global_transform: Transform2D

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

  look_at(point: Vector2): void
  to_global(local_point: Vector2): Vector2
  to_local(global_point: Vector2): Vector2
  translate(offset: Vector2): void
  global_translate(offset: Vector2): void
  rotate(radians: number): void
  get_angle_to(point: Vector2): number
  apply_scale(ratio: Vector2): void
  move_local_x(delta: number, scaled?: boolean): void
  move_local_y(delta: number, scaled?: boolean): void
  get_relative_transform_to_parent(parent: GodotNode): Transform2D
}

// ===== ControlBase =====

interface ControlBase extends CanvasItemBase {
  position: Vector2
  size: Vector2
  custom_minimum_size: Vector2
  rotation: number
  rotation_degrees: number
  scale: Vector2
  pivot_offset: Vector2
  global_position: Vector2

  anchor_left: number
  anchor_right: number
  anchor_top: number
  anchor_bottom: number
  offset_left: number
  offset_right: number
  offset_top: number
  offset_bottom: number
  grow_horizontal: number
  grow_vertical: number
  size_flags_horizontal: number
  size_flags_vertical: number
  size_flags_stretch_ratio: number

  focus_mode: number
  mouse_filter: number
  mouse_force_pass: boolean
  mouse_default_cursor_shape: number
  tooltip_text: string
  clip_contents: boolean
  auto_translate: boolean
  layout_direction: number
  theme: GodotResource | null
  theme_type_variation: string

  set_anchor(side: number, value: number): void
  get_anchor(side: number): number
  set_begin(pos: Vector2): void
  set_end(pos: Vector2): void
  get_begin(): Vector2
  get_end(): Vector2
}

// ===== Node3DBase =====

interface Node3DBase extends GodotNodeBase {
  position: Vector3
  rotation: Vector3
  scale: Vector3
  global_position: Vector3
  global_rotation: Vector3
  global_scale: Vector3
  transform: Transform3D
  global_transform: Transform3D
  basis: any
  global_basis: any
  quaternion: any
  top_level: boolean
  visible: boolean
  rotation_edit_mode: number
  rotation_order: number

  show(): void
  hide(): void
  is_visible_in_tree(): boolean

  look_at(target: Vector3, up?: Vector3): void
  look_at_from_position(pos: Vector3, target: Vector3, up?: Vector3): void
  translate(offset: Vector3): void
  global_translate(offset: Vector3): void
  rotate(axis: Vector3, angle: number): void
  rotate_x(angle: number): void
  rotate_y(angle: number): void
  rotate_z(angle: number): void
  global_rotate(axis: Vector3, angle: number): void
  to_global(local_point: Vector3): Vector3
  to_local(global_point: Vector3): Vector3
  orthonormalize(): void
  set_identity(): void
  set_disable_scale(disable: boolean): void
  is_scale_disabled(): boolean
  rotate_object_local(axis: Vector3, angle: number): void
  scale_object_local(scale: Vector3): void
  translate_object_local(offset: Vector3): void
  force_update_transform(): void
  set_notify_transform(enable: boolean): void
  set_notify_local_transform(enable: boolean): void
  is_transform_notification_enabled(): boolean
  set_ignore_transform_notification(enabled: boolean): void
  get_parent_node_3d(): Node3D | null
  get_world_3d(): any
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
  region_rect: Rect2
  region_filter_clip_enabled: boolean

  get_rect(): Rect2
  is_pixel_opaque(pos: Vector2): boolean
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
  up_direction: Vector2
  motion_mode: number
  max_slides: number
  floor_max_angle: number
  floor_snap_length: number
  floor_stop_on_slope: boolean
  floor_block_on_wall: boolean
  floor_constant_speed: boolean
  slide_on_ceiling: boolean
  wall_min_slide_angle: number
  safe_margin: number
  platform_floor_layers: number
  platform_wall_layers: number
  platform_on_leave: number

  move_and_slide(): void
  is_on_floor(): boolean
  is_on_ceiling(): boolean
  is_on_wall(): boolean
  is_on_floor_only(): boolean
  is_on_ceiling_only(): boolean
  is_on_wall_only(): boolean
  get_floor_normal(): Vector2
  get_floor_angle(up_direction?: Vector2): number
  get_wall_normal(): Vector2
  get_last_motion(): Vector2
  get_real_velocity(): Vector2
  get_platform_velocity(): Vector2
  get_position_delta(): Vector2
  get_last_slide_collision(): any
  get_slide_collision(idx: number): any
  get_slide_collision_count(): number
  apply_floor_snap(): void
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
  continuous_cd: number
  freeze: boolean
  freeze_mode: number
  gyroscopic_torque: number
  center_of_mass_mode: number
  physics_material_override: GodotResource | null

  apply_central_force(force: Vector2): void
  apply_force(force: Vector2, position: Vector2): void
  apply_impulse(impulse: Vector2, position?: Vector2): void
  apply_central_impulse(impulse: Vector2): void
  apply_torque(torque: number): void
  apply_torque_impulse(torque: number): void
  set_linear_velocity(vel: Vector2): void
  set_angular_velocity(vel: number): void
  get_colliding_bodies(): GodotNode[]
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
  gravity_space_override: number
  gravity_point_center: Vector2
  linear_damp: number
  angular_damp: number
  linear_damp_space_override: number
  angular_damp_space_override: number
  priority: number
  monitorable: boolean
  monitoring: boolean
  audio_bus_override: boolean
  audio_bus_name: string
  collision_mask: number
  collision_layer: number

  has_overlapping_bodies(): boolean
  has_overlapping_areas(): boolean
  get_overlapping_bodies(): GodotNode[]
  get_overlapping_areas(): GodotNode[]
  get_overlapping_bodies_count(): number
  get_overlapping_areas_count(): number
  overlaps_area(area: GodotNode): boolean
  overlaps_body(body: GodotNode): boolean
}

// --- CollisionShape2D ---

interface CollisionShape2D extends Node2DBase {
  class_type: "CollisionShape2D"

  disabled: boolean
  one_way_collision: boolean
  one_way_collision_margin: number
}

// --- CollisionPolygon2D ---

interface CollisionPolygon2D extends Node2DBase {
  class_type: "CollisionPolygon2D"

  depth: number
  disabled: boolean
  one_way_collision: boolean
  one_way_collision_margin: number
  polygon: Vector2[]
}

// --- Camera2D ---

interface Camera2D extends Node2DBase {
  class_type: "Camera2D"

  anchor_mode: number
  zoom: Vector2
  offset: Vector2
  enabled: boolean
  current: boolean
  limit_smoothed: Vector2
  limit_left: number
  limit_right: number
  limit_top: number
  limit_bottom: number
  drag_margin_left: number
  drag_margin_right: number
  drag_margin_top: number
  drag_margin_bottom: number
  position_smoothing_enabled: boolean
  position_smoothing_speed: number
  drag_horizontal_enabled: boolean
  drag_vertical_enabled: boolean
  drag_margin_h_enabled: boolean
  drag_margin_v_enabled: boolean
  ignore_rotation: boolean
  screen_drawing_enabled: boolean

  make_current(): void
  clear_current(): void
  is_current(): boolean
  get_screen_center_position(): Vector2
  align(): void
  force_update_scroll(): void
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
  root_node: string
  playing: boolean
  playback_process_mode: number
  method_call_mode: number
  reset_on_save: boolean

  play(name?: string): void
  stop(keep_state?: boolean): void
  pause(): void
  seek(seconds: number, update?: boolean): void
  is_playing(): boolean
  get_current_animation(): string
  get_animation(name: string): any
  get_animation_list(): string[]
  clear_queue(): void
  queue(name: string): void
  get_queue(): string[]
  advance(delta: number): void
  set_blend_time(anim1: string, anim2: string, sec: number): void
  get_blend_time(anim1: string, anim2: string): number
  get_playing_speed(): number
}

// --- AudioStreamPlayer2D ---

interface AudioStreamPlayer2D extends Node2DBase {
  class_type: "AudioStreamPlayer2D"

  stream: GodotResource | null
  volume_db: number
  pitch_scale: number
  playing: boolean
  autoplay: boolean
  max_distance: number
  attenuation: number
  panning_strength: number
  bus: string
  max_polyphony: number

  play(from_position?: number): void
  stop(): void
  is_playing(): boolean
  get_playback_position(): number
  seek(seconds: number): void
}

// --- RayCast2D ---

interface RayCast2D extends Node2DBase {
  class_type: "RayCast2D"

  enabled: boolean
  hit_from_inside: boolean
  target_position: Vector2
  collide_with_bodies: boolean
  collide_with_areas: boolean
  collision_mask: number

  is_colliding(): boolean
  get_collider(): GodotNode | null
  get_collision_normal(): Vector2
  get_collision_point(): Vector2
  get_collision_face_index(): number
  force_raycast_update(): void
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
  draw_order: number
  interpolate: boolean
  collision_enabled: boolean
  collision_mode: number
  collision_base_size: number
  collision_mask: number
  sub_emitter: GodotNode | null
  attractor_interaction_enabled: boolean

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
  draw_order: number
  collision_enabled: boolean
  collision_mode: number
  collision_base_size: number
  collision_mask: number

  restart(): void
  set_emitting(emitting: boolean): void
}

// --- Node2D ---

interface Node2D extends Node2DBase {
  class_type: "Node2D"
}

// --- Marker2D ---

interface Marker2D extends Node2DBase {
  class_type: "Marker2D"
  gizmo_extents: number
}

// --- Path2D ---

interface Path2D extends Node2DBase {
  class_type: "Path2D"
  curve: any
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
}

// --- TileMap ---

interface TileMap extends Node2DBase {
  class_type: "TileMap"

  tile_set: GodotResource | null
  rendering_quadrant_size: number
  collision_animation_enabled: boolean
  cell_quadrant_size: number

  set_cell(layer: number, coords: Vector2, source_id: number, atlas_coords?: Vector2, alternative_tile?: number): void
  get_cell_source_id(layer: number, coords: Vector2): number
  get_cell_atlas_coords(layer: number, coords: Vector2): Vector2
  set_cells_terrain_connect(layer: number, cells: Vector2[], terrain_set: number, terrain: number, ignore_existing?: boolean): void
  get_used_cells(layer: number): Vector2[]
  get_used_cells_by_id(layer: number, source_id?: number, atlas_coords?: Vector2, alternative_tile?: number): Vector2[]
  get_used_rect(): Rect2
  clear_layer(layer: number): void
  clear(): void
}

// --- TileMapLayer ---

interface TileMapLayer extends Node2DBase {
  class_type: "TileMapLayer"

  tile_set: GodotResource | null
  enabled: boolean

  set_cell(coords: Vector2, source_id: number, atlas_coords?: Vector2, alternative_tile?: number): void
  get_cell_source_id(coords: Vector2): number
  get_cell_atlas_coords(coords: Vector2): Vector2
  get_cell_alternative_tile(coords: Vector2): number
  get_cell_tile_data(coords: Vector2): any
  get_used_cells(): Vector2[]
  get_used_rect(): Rect2
  clear(): void
}

// --- VisibleOnScreenNotifier2D ---

interface VisibleOnScreenNotifier2D extends Node2DBase {
  class_type: "VisibleOnScreenNotifier2D"

  rect: Rect2

  is_on_screen(): boolean
}

// --- VisibleOnScreenEnabler2D ---

interface VisibleOnScreenEnabler2D extends Node2DBase {
  class_type: "VisibleOnScreenEnabler2D"

  enable_mode: number
  rect: Rect2
}

// --- MeshInstance2D ---

interface MeshInstance2D extends Node2DBase {
  class_type: "MeshInstance2D"
  texture: GodotTexture | null
}

// --- NavigationAgent2D ---

interface NavigationAgent2D extends Node2DBase {
  class_type: "NavigationAgent2D"

  target_position: Vector2
  navigation_layers: number
  path_desired_distance: number
  target_desired_distance: number
  path_max_distance: number
  velocity: Vector2
  max_speed: number

  is_navigation_finished(): boolean
  get_next_path_position(): Vector2
  get_current_navigation_result(): any
}

// --- NavigationObstacle2D ---

interface NavigationObstacle2D extends Node2DBase {
  class_type: "NavigationObstacle2D"

  radius: number
  velocity: Vector2
}

// --- NavigationRegion2D ---

interface NavigationRegion2D extends Node2DBase {
  class_type: "NavigationRegion2D"

  enabled: boolean
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
}

// ==========================================
//            CONTROL TYPES
// ==========================================

// --- Control ---

interface Control extends ControlBase {
  get_parent_area_size(): Vector2
  get_minimum_size(): Vector2
  get_combined_minimum_size(): Vector2
  set_anchor_and_offset(...args: any[]): void

  grab_focus(): void
  release_focus(): void
  has_focus(): boolean
  accept_event(): void
  get_focus_owner(): GodotNode | null
  find_next_valid_focus(): GodotNode | null
  find_prev_valid_focus(): GodotNode | null

  get_rect(): Rect2
  get_global_rect(): Rect2
  get_screen_position(): Vector2
  has_point(point: Vector2): boolean
  warp_mouse(position: Vector2): void
}

// --- Label ---

interface Label extends Omit<Control, 'class_type'> {
  class_type: "Label"

  text: string
  horizontal_alignment: number
  vertical_alignment: number
  autowrap_mode: number
  clip_text: boolean
  max_lines_visible: number
  language: string

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
  icon_alignment: number
}

// --- TextureButton ---

interface TextureButton extends Omit<Control, 'class_type'> {
  class_type: "TextureButton"

  texture_normal: GodotTexture | null
  texture_pressed: GodotTexture | null
  texture_hover: GodotTexture | null
  texture_disabled: GodotTexture | null
  texture_focused: GodotTexture | null
  expand: boolean
  stretch_mode: number
  flip_h: boolean
  flip_v: boolean
  ignore_texture_size: boolean
}

// --- TextureRect ---

interface TextureRect extends Omit<Control, 'class_type'> {
  class_type: "TextureRect"

  texture: GodotTexture | null
  expand_mode: number
  stretch_mode: number
  flip_h: boolean
  flip_v: boolean
}

// --- ColorRect ---

interface ColorRect extends Omit<Control, 'class_type'> {
  class_type: "ColorRect"

  color: Color
}

// --- RichTextLabel ---

interface RichTextLabel extends Omit<Control, 'class_type'> {
  class_type: "RichTextLabel"

  text: string
  bbcode_enabled: boolean
  bbcode_text: string

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

interface LineEdit extends Omit<Control, 'class_type'> {
  class_type: "LineEdit"

  text: string
  placeholder_text: string
  placeholder_alignment: number
  editable: boolean
  max_length: number
  caret_column: number
  readonly: boolean
  select_all_on_focus: boolean

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

  get_item_text(index: number): string
  select(index: number): void
  add_item(text: string, icon?: GodotTexture): void
  add_separator(): void
  clear(): void
  get_popup(): any
}

// --- CheckButton / CheckBox ---

interface CheckButton extends Omit<Button, 'class_type'> {
  class_type: "CheckButton"
}

interface CheckBox extends Omit<Button, 'class_type'> {
  class_type: "CheckBox"
}

// --- SpinBox ---

interface SpinBox extends Omit<Control, 'class_type'> {
  class_type: "SpinBox"

  value: number
  min_value: number
  max_value: number
  step: number
  prefix: string
  suffix: string
  editable: boolean
}

// --- HSlider / VSlider ---

interface HSlider extends Omit<Control, 'class_type'> {
  class_type: "HSlider"

  value: number
  min_value: number
  max_value: number
  step: number
  tick_count: number
}

interface VSlider extends Omit<Control, 'class_type'> {
  class_type: "VSlider"

  value: number
  min_value: number
  max_value: number
  step: number
  tick_count: number
}

// --- ProgressBar ---

interface ProgressBar extends Omit<Control, 'class_type'> {
  class_type: "ProgressBar"

  value: number
  min_value: number
  max_value: number
  show_percentage: boolean
}

// --- LinkButton ---

interface LinkButton extends Omit<Button, 'class_type'> {
  class_type: "LinkButton"

  uri: string
}

// --- PopupMenu ---

interface PopupMenu extends Omit<Control, 'class_type'> {
  class_type: "PopupMenu"

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

interface ItemList extends Omit<Control, 'class_type'> {
  class_type: "ItemList"

  max_columns: number
  same_column_width: boolean
  allow_reselect: boolean
  auto_height: boolean

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

interface Tree extends Omit<Control, 'class_type'> {
  class_type: "Tree"

  columns: number
  hide_root: boolean

  clear(): void
  get_root(): any
  create_item(parent?: any): any
  get_selected(): any
}

// --- TabContainer ---

interface TabContainer extends Omit<Control, 'class_type'> {
  class_type: "TabContainer"

  current_tab: number
  tabs_count: number
}

// --- Window ---

interface Window extends Omit<Control, 'class_type'> {
  class_type: "Window"

  title: string
  size: Vector2

  close_request(): void
}

// --- Container ---

interface Container extends Omit<Control, 'class_type'> {
  class_type: "Container"
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
}

interface CenterContainer extends Omit<Container, 'class_type'> {
  class_type: "CenterContainer"
  use_top_level: boolean
}

interface AspectRatioContainer extends Omit<Container, 'class_type'> {
  class_type: "AspectRatioContainer"
  ratio: number
  stretch_mode: number
  alignment_h: number
  alignment_v: number
}

interface ScrollContainer extends Omit<Container, 'class_type'> {
  class_type: "ScrollContainer"

  scroll_horizontal: number
  scroll_vertical: number
  scroll_horizontal_enabled: boolean
  scroll_vertical_enabled: boolean
}

interface SplitContainer extends Omit<Container, 'class_type'> {
  class_type: "SplitContainer" | "HSplitContainer" | "VSplitContainer"

  split_offset: number
  dragger_visibility: number
  collapsed: boolean
  vertical: boolean
}

interface HSplitContainer extends Omit<SplitContainer, 'class_type'> {
  class_type: "HSplitContainer"
}

interface VSplitContainer extends Omit<SplitContainer, 'class_type'> {
  class_type: "VSplitContainer"
}

interface MenuBar extends Omit<Control, 'class_type'> {
  class_type: "MenuBar"
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
  up_direction: Vector3
  motion_mode: number
  max_slides: number
  floor_max_angle: number
  floor_stop_on_slope: boolean
  floor_block_on_wall: boolean
  slide_on_ceiling: boolean
  safe_margin: number

  move_and_slide(): void
  is_on_floor(): boolean
  is_on_ceiling(): boolean
  is_on_wall(): boolean
  is_on_floor_only(): boolean
  is_on_ceiling_only(): boolean
  is_on_wall_only(): boolean
  get_floor_normal(): Vector3
  get_wall_normal(): Vector3
  get_last_motion(): Vector3
  get_real_velocity(): Vector3
  get_platform_velocity(): Vector3
  get_last_slide_collision(): any
  get_slide_collision(idx: number): any
  get_slide_collision_count(): number
}

interface RigidBody3D extends Node3DBase {
  class_type: "RigidBody3D"

  mass: number
  gravity_scale: number
  linear_velocity: Vector3
  angular_velocity: Vector3
  inertia: number
  can_sleep: boolean
  sleeping: boolean
  linear_damp: number
  angular_damp: number
  constant_force: Vector3
  constant_torque: Vector3
  freeze: boolean
  freeze_mode: number
  locked_rotates: boolean
  continuous_cd: number
  physics_material_override: GodotResource | null

  apply_central_force(force: Vector3): void
  apply_force(force: Vector3, position: Vector3): void
  apply_impulse(impulse: Vector3, position?: Vector3): void
  apply_central_impulse(impulse: Vector3): void
  apply_torque(torque: Vector3): void
  apply_torque_impulse(torque: Vector3): void
  set_linear_velocity(vel: Vector3): void
  set_angular_velocity(vel: Vector3): void
  get_colliding_bodies(): GodotNode[]
}

interface StaticBody3D extends Node3DBase {
  class_type: "StaticBody3D"
}

interface Area3D extends Node3DBase {
  class_type: "Area3D"

  gravity: number
  gravity_direction: Vector3
  gravity_is_point: boolean
  gravity_point_unit_distance: number
  gravity_point_center: Vector3
  gravity_space_override: number
  linear_damp: number
  angular_damp: number
  linear_damp_space_override: number
  angular_damp_space_override: number
  priority: number
  monitoring: boolean
  monitorable: boolean
  collision_mask: number
  collision_layer: number
  audio_bus_override: boolean
  audio_bus_name: string

  has_overlapping_bodies(): boolean
  has_overlapping_areas(): boolean
  get_overlapping_bodies(): GodotNode[]
  get_overlapping_areas(): GodotNode[]
  get_overlapping_bodies_count(): number
  get_overlapping_areas_count(): number
  overlaps_area(area: GodotNode): boolean
  overlaps_body(body: GodotNode): boolean
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
  far: number
  near: number
  fov: number
  size: number
  projection: number
  cull_mask: number
  h_offset: number
  v_offset: number

  make_current(): void
  clear_current(): void
  is_current(): boolean
  get_camera_rid(): any
  get_camera_transform(): Transform3D
  project_ray_normal(screen_point: Vector2): Vector3
  project_position(screen_point: Vector2, depth: number): Vector3
  unproject_position(global_point: Vector3): Vector2
}

interface Path3D extends Node3DBase {
  class_type: "Path3D"
  curve: any
}

interface PathFollow3D extends Node3DBase {
  class_type: "PathFollow3D"
  progress: number
  progress_ratio: number
  rotation_enabled: boolean
}

interface MeshInstance3D extends Node3DBase {
  class_type: "MeshInstance3D"

  mesh: GodotResource | null
  material_override: GodotMaterial | null
  skeleton: GodotNode | null

  get_surface_override_material(surface: number): GodotMaterial | null
  set_surface_override_material(surface: number, material: GodotMaterial | null): void
}

interface AudioStreamPlayer3D extends Node3DBase {
  class_type: "AudioStreamPlayer3D"

  stream: GodotResource | null
  volume_db: number
  pitch_scale: number
  playing: boolean
  autoplay: boolean
  max_distance: number
  attenuation: number
  unit_size: number
  emission_angle: number
  emission_angle_enabled: boolean
  bus: string
  max_polyphony: number

  play(from_position?: number): void
  stop(): void
  is_playing(): boolean
  get_playback_position(): number
  seek(seconds: number): void
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
  | Timer
  | AnimationPlayer
  | CanvasLayer
  | AudioStreamPlayer2D
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
  GodotNodeBase

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
}

interface GodotMaterial extends GodotObjectBase {
  readonly class_type: MaterialClassType
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
