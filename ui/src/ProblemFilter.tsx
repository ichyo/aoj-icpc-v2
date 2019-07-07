import { Problem, User } from './model';

export default class ProblemFilter {
    readonly minimumPoint: number | null;
    readonly maximumPoint: number | null;
    readonly hideAC: boolean;
    readonly showPending: boolean;
    readonly sinceYear: number | null;
    readonly untilYear: number | null;

    constructor(builder: ProblemFilterBuilder) {
        this.minimumPoint = builder.minimumPoint;
        this.maximumPoint = builder.maximumPoint;
        this.hideAC = builder.hideAC;
        this.showPending = builder.showPending;
        this.sinceYear = builder.sinceYear;
        this.untilYear = builder.untilYear;
    }

    static default(): ProblemFilter {
        return new ProblemFilter(new ProblemFilterBuilder())
    }

    static builder(): ProblemFilterBuilder {
        return new ProblemFilterBuilder();
    }

    filters(problems: Problem[], u: User | null): Problem[] {
        const solved_set = new Set(u ? u.solutions : []);
        return problems.filter(p =>
            (!this.minimumPoint || p.point >= this.minimumPoint)
            && (!this.maximumPoint || p.point <= this.maximumPoint)
            && (!this.sinceYear || p.year >= this.sinceYear)
            && (!this.untilYear || p.year <= this.untilYear)
            && (!this.hideAC || !solved_set.has(p.id))
            && (p.status === "active" ||
                (this.showPending && p.status === "pending"))
        )
    }
}

class ProblemFilterBuilder {
    private _minimumPoint: number | null = null;
    private _maximumPoint: number | null = null;
    private _sinceYear: number | null = null;
    private _untilYear: number | null = null;
    private _hideAC: boolean = false;
    private _showPending: boolean = false;

    setMinimumPoint(point: number | null): ProblemFilterBuilder {
        this._minimumPoint = point;
        return this;
    }

    setMaximumPoint(point: number | null): ProblemFilterBuilder {
        this._maximumPoint = point;
        return this;
    }

    setSinceYear(point: number | null): ProblemFilterBuilder {
        this._sinceYear = point;
        return this;
    }

    setUntilYear(point: number | null): ProblemFilterBuilder {
        this._untilYear = point;
        return this;
    }

    setHideAC(hideAC: boolean): ProblemFilterBuilder {
        this._hideAC = hideAC;
        return this;
    }

    setShowPending(showPending: boolean): ProblemFilterBuilder {
        this._showPending = showPending;
        return this;
    }

    build(): ProblemFilter {
        return new ProblemFilter(this);
    }

    get minimumPoint() {
        return this._minimumPoint;
    }

    get maximumPoint() {
        return this._maximumPoint;
    }

    get sinceYear() {
        return this._sinceYear;
    }

    get untilYear() {
        return this._untilYear;
    }

    get hideAC() {
        return this._hideAC;
    }

    get showPending() {
        return this._showPending;
    }
}
