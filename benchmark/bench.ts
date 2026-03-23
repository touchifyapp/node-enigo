import { Bench } from "tinybench";

import { Enigo } from "../index.js";

const bench = new Bench();

bench.add("Construct enigo", () => {
    Enigo.create();
});

let mv_enigo: Enigo;
bench.add(
    "move mouse",
    () => {
        mv_enigo.mouseMove(Math.round(Math.random() * 400), Math.round(Math.random() * 400));
    },
    {
        beforeAll() {
            mv_enigo = Enigo.create();
        },
    },
);

await bench.run();

console.table(bench.table());
