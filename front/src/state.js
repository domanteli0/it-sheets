import { RangeIter } from './common.js';
import { Coordinate } from './coordinate.js';
import 'https://cdnjs.cloudflare.com/ajax/libs/jquery/3.7.1/jquery.js';

let initState = {
    cols: 10,
    rows: 10,
    socket: null,
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

            cell.id  = new Coordinate(row_no, col_no).toId();
        });
    });

    // it seems that only safari supports bare "/ws" url
    let socket = new WebSocket("ws://localhost:3000/ws");

    socket.onmessage = (event) => {
        let eventData = JSON.parse(event.data);
        let row = eventData.coordinate.row;
        let col = eventData.coordinate.col;

        $("#" + new Coordinate(row, col).toId()).html(eventToNode(eventData));
    }

    initState.socket = socket;


    return initState;
}

