
function RangeIter(till) {

    return Array(till).fill(0).map((_, i) => i);
}

// const expendObj = (name, proc) => {
//     Object.defineProperty(Object.prototype, name, {
//         value: proc,
//         writable: true,
//         configurable: true,
//     })
// }

// const expendObjWithDoTimes = () => {
//     expendObj("doTimes", (times, proc) => {
//         () => { RangeIter(times).forEach(proc) }
//     })
// }

export {
    RangeIter,
    // expendObjWithDoTimes
}