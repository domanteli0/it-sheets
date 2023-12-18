import { RangeIter } from './common.js';
import { Coordinate } from './coordinate.js';
import 'https://cdnjs.cloudflare.com/ajax/libs/jquery/3.7.1/jquery.js';

let initState = {
    cols: 10,
    rows: 10,
}

const eventToNode = (eventData) => {
    // TODO: actual logic
    return document.createTextNode(eventData.text);
}

export function init() {
    let table = document.getElementById("mainTable");

    RangeIter(initState.rows).forEach((row_no) => {
        let row = table.insertRow(row_no);
        RangeIter(initState.cols).forEach((col_no) => {
            let cell = row.insertCell(col_no);

            cell.id = new Coordinate(row_no, col_no).toId();
        });
    });

    setInterval(() => {
        $.get({
            url: '/poll_state',
            success: (data) => {
                let updates = JSON.parse(data).updates;

                updates.forEach((cellUpdate) => {

                    let row = cellUpdate.coordinate.row;
                    let col = cellUpdate.coordinate.col;

                    $("#" + new Coordinate(row, col).toId()).html(eventToNode(cellUpdate));
                })
            }
        })
    }, 10);


    return initState;
}

