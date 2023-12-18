export class Coordinate {
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