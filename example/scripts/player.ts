export default class {
  jsnode: GodotNode
  node: Node2DBase
  id = 0
  constructor(node: GodotNode) {
    this.jsnode = node
    this.node = node.parent! as Node2DBase
  }

  onReady() {
    this.id = setInterval(() => console.log("Interval", this.jsnode.name), 500)
  }

  onProcess(dt: number) {
    this.node.rotation += 10 * dt
    if (this.node.rotation > 10) {
      this.jsnode.queue_free()
    }
  }

  onDestroy() { clearInterval(this.id) }
}
