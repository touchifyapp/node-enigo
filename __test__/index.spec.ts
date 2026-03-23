import test from "ava";

import { Enigo } from "../index.js";

test.serial("Construct Enigo", (t) => {
    const enigo = Enigo.create();
    t.true(enigo instanceof Enigo, "result should be instance of Enigo");
});

test.serial("get mouse position", (t) => {
    const enigo = Enigo.create();
    const [x, y] = enigo.getMousePosition();
    t.true(x >= 0 && y >= 0, "x and y should be >= 0");
});

test.serial("move mouse", (t) => {
    const enigo = Enigo.create();
    const [x, y] = enigo.getMousePosition();
    enigo.mouseMove(x + 10, y + 10);
    const [new_x, new_y] = enigo.getMousePosition();

    t.is(new_x, x + 10);
    t.is(new_y, y + 10);
});
