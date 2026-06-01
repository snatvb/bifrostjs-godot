import engine from "bifrostjs"
import { log } from "../test"
import { sum } from "./typemagic"

export default class JustNode {
  child: Node2D
  constructor(private node: GodotNode) {
    // this.node = node;
    this.child = this.node.get_child(0) as Node2D
    console.log(`subscribe ${this.child.name}`)
    this.child.connect("boom", node => console.log(node.name))


    const button = this.node.parent.parent.get_child(1).get_child(0).get_child(0)
    // console.log("hello", this.node.parent.parent.get_child(1).get_child(0).get_child(0).class_type);

    let counter = 0
    const id = button.connect("button_down", function buttonDown(...args) {
      console.log("down");
      setTimeout(() => console.log("TIMEOUT"), 1000)
      counter++;
      if (counter > 15) {
        console.log("disconnected", id, ...args);
        button.disconnect(id);
      }
    })
  }

  onReady() {
    console.trace("Ready", this.node.parent?.name)
    this.node.register_signal("test_js_signal")
    this.node.emit_signal("test_js_signal", [123])
    log()

    const node2 = engine.createNode("Node2D")
    node2.name = "Test created"

    console.log("created", node2.name)
    const player = engine.instantiate<Node2D>("uid://bctt6aspp070r")
    player.position.x = 128
    player.position.y = 128
    this.node.add_child(node2)
    this.node.parent.parent.add_child(player)
    // console.log("ready", this.node.id, [1, 2, "hi"], null, undefined, NaN);
    // console.error(new Error("Failed"), "asd")
  }

  onProcess() {
    (this.node.parent.get_child(1) as Node2D).position.x += 1
    // this.node.parent.position.x += 1
    // this.pos.x += 1
    // this.node.parent.position = this.pos
    // console.log(this.node.parent.position.x)
  }
}
