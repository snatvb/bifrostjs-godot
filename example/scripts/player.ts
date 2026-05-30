export default class {
  jsnode: GodotNode
  node: GodotNode
  constructor(node: GodotNode) {
    this.jsnode = node
    this.node = node.parent!
  }

  onProcess(dt: number) {
    this.node.rotation += 10 * dt
    if (this.node.rotation > 10) {
      this.jsnode.queue_free()
    }
  }
}
