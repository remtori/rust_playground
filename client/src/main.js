const canvas = document.getElementById('screen');
const context = canvas.getContext('2d');

context.fillStyle = '#fff';
context.fillRect(0, 0, canvas.width, canvas.height);

const remote = new WebSocket(`ws://${location.host}/ws/`);

let isPressed = false;
canvas.addEventListener('mousedown', e => {
    if (e.button === 0)
        isPressed = true;
});

canvas.addEventListener('mouseup', e => {
    if (e.button == 0)
        isPressed = false;
});

canvas.addEventListener('mouseleave', e => {
    isPressed = false;
});

canvas.addEventListener('mousemove', e => {
    if (isPressed) {
        context.fillStyle = '#000';
        context.fillRect(e.offsetX, e.offsetY, 2, 2);
        remote.send(new Uint32Array([ e.offsetX, e.offsetY ]));
    }
});

remote.onmessage = e => {
    e.data.arrayBuffer().then(buffer => {
        const view = new Uint32Array(buffer);
        context.fillStyle = '#f00';
        context.fillRect(view[0], view[1], 2, 2);
    })
}
