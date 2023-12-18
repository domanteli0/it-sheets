
function RangeIter(till) {

    return Array(till).fill(0).map((_, i) => i);
}

export {
    RangeIter,
}