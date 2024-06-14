class DatePickerComponents {
    constructor(rawValue) {
        this.rawValue = rawValue;
    }

    static get hourAndMinute() {
        return new DatePickerComponents(1);
    }

    static get date() {
        return new DatePickerComponents(2);
    }

    // eq
    equals(other) {
        return this.rawValue === other.rawValue;
    }
}

export default DatePickerComponents;
