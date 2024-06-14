class _SwiftDate {
    constructor(date) {
        this.date = date ?? new Date();
    }
}

function SwiftDate() {
    return new _SwiftDate();
}

SwiftDate.fromTimestamp = function (timestamp) {
    return new _SwiftDate(new Date(timestamp));
}

export default SwiftDate;