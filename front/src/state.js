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
    let table = $("#mainTable");

    RangeIter(initState.rows).forEach((row_no) => {
        let row = $("<tr>")
        RangeIter(initState.cols).forEach((col_no) => {
            let cell = $("<td>").attr('id', new Coordinate(row_no, col_no).toId())
            row.append(cell);
        });
        table.append(row);
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
            },
            error: () => console.error("Failed to poll state"),
        })
    }, 2000);

    return initState;
}

export function sendUpdate(
    coordinate,
    text
) {

    let payload = JSON.stringify({
        coordinate: {
            row: coordinate.row,
            col: coordinate.col,
        },
        text: text,
    });

    $.post({
        url: '/update',
        method: 'POST',
        contentType: "application/json",
        data: payload
    });
}