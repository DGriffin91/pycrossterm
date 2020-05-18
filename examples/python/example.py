# copy "..\..\target\debug\pycrossterm.dll" "pycrossterm.pyd" /Y
import time
import atexit
from pycrossterm import cursor, event, style, terminal, attribute, StyledContent

color = style.color
named_color = style.named_color


def exit_handler():
    style.set_foreground_color(255, 255, 255)
    cursor.enable_blinking()
    cursor.show()


atexit.register(exit_handler)

event.enable_mouse_capture()


style.set_foreground_color(128, 128, 0)
print("!!!")
style.set_foreground_color(255, 128, 0)
print("!!!")
style.set_foreground_color(255, 255, 255)
print("!!!")
StyledContent("TEST TEST TEST\nTEST TEST").color(color(255, 0, 255)).on(
    color(50, 50, 50)
).attribute(
    attribute.CrossedOut()  # Wont work in windows CMD
).print()
e = event.read()
print(e)


terminal.clear("All")
cursor.disable_blinking()
while True:
    cursor.hide()
    e = event.read()
    if e.event == "MouseDown":
        style.set_foreground_color(255, 0, 0)
        cursor.move_to(e.x, e.y)
        print("P")
    elif e.event == "MouseUp":
        cursor.move_to(e.x, e.y)
        StyledContent("X").color(named_color("Black")).on(named_color("White")).print()
    elif e.event == "MouseDrag":
        cursor.move_to(e.x, e.y)
        style.new("X").color(color(0, 0, 255)).on(color(50, 50, 50)).print()
