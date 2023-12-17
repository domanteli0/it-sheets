import { RangeIter } from './common.js';

let initState = {
    cols: 5,
    rows: 5,
}

export function init() {

    let table = document.getElementById("mainTable");
    RangeIter(initState.rows).forEach((i) => {
        let row = table.insertRow(0);
        RangeIter(initState.cols).forEach((i) => row.insertCell(i));
    });

    return initState;
}
