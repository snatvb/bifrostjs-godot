extends Node

signal boom(Node)

func _on_timer_timeout() -> void:
	print("Timeout")
	boom.emit(self)
