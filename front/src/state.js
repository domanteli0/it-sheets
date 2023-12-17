import { RangeIter } from './common.js';
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
        let row = table.insertRow(0);
        RangeIter(initState.cols).forEach((col_no) => {
            let cell = row.insertCell(col_no);

            cell.id = row_no.toString() + "-" + col_no.toString();
        });
    });

    let socket = new WebSocket('/ws');

    socket.onmessage = (event) => {
        alert(event.data.toString());
        let eventData = JSON.parse(event.data);

        $("#" + eventData.coordinates.row + "-" + eventData.coordinates.col).html(eventToNode(eventData));
    }

    initState.socket = socket;


    return initState;
}
