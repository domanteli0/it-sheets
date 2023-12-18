import('https://cdnjs.cloudflare.com/ajax/libs/jquery/3.7.1/jquery.js');

function RangeIter(till) {

    return Array(till).fill(0).map((_, i) => i);
}

class Coordinate {
    static fromId(id) {
        let coordinates = id.toString().split('-');
        console.log(coordinates);
        return new this(Number(coordinates[0]), Number(coordinates[1]));
    }

    constructor(row, col) {
        this.row = row;
        this.col = col;
    }

    toId() {
        return this.row.toString() + '-' + this.col.toString();
    }
}


let initState = {
    cols: 10,
    rows: 10,
}

const eventToNode = (eventData) => {
    // TODO: actual logic
    return document.createTextNode(eventData.text);
}

function init() {
    let table = $("#mainTable");

    // [3.d]
    // Dinamiškai sukuriama pradinė lentelė; įterpiamos naujos tr ir td žymės
    RangeIter(initState.rows).forEach((row_no) => {
        let row = $("<tr>")
        RangeIter(initState.cols).forEach((col_no) => {
            let cell = $("<td>").attr('id', new Coordinate(row_no, col_no).toId())
            row.append(cell);
        });
        table.append(row);
    });

    setInterval(() => {
		// [4.b 2/2]
		// kas kurį laiką gaunami nauji lentelės duomenys
        $.get({
            url: '/poll_state',
            success: (data) => {
                let updates = JSON.parse(data).updates;

                updates.forEach((cellUpdate) => {

                    let row = cellUpdate.coordinate.row;
                    let col = cellUpdate.coordinate.col;

                    // [3.a 2/2] [4.c 2/2] tekstinis turinys pakeičiamas kai gaunamas atnaujinimas iš serverio
                    $("#" + new Coordinate(row, col).toId()).html(eventToNode(cellUpdate));
                })
            },
            error: () => console.error("Failed to poll state"),
        })
    }, 200);

    return initState;
}

// [4.a 2/2]
// patalpinami nauji duomenys
function sendUpdate(
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