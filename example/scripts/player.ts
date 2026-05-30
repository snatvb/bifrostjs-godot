export default class {
  jsnode: GodotNode
  node: Node2DBase
  constructor(node: GodotNode) {
    this.jsnode = node
    this.node = node.parent! as Node2DBase
  }

  onProcess(dt: number) {
    this.node.rotation += 10 * dt
    if (this.node.rotation > 10) {
      this.jsnode.queue_free()
    }
  }
}
