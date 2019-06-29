import { Problem, User } from './model';

export default class ProblemFilter {
    readonly minimumPoint: number | null;
    readonly maximumPoint: number | null;
    readonly hideAC: boolean;

    constructor(builder: ProblemFilterBuilder) {
        this.minimumPoint = builder.minimumPoint;
        this.maximumPoint = builder.maximumPoint;
        this.hideAC = builder.hideAC;
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
            && (!this.hideAC || !solved_set.has(p.id))
        )
    }
}

class ProblemFilterBuilder {
    private _minimumPoint: number | null = null;
    private _maximumPoint: number | null = null;
    private _hideAC: boolean = false;

    setMinimumPoint(point: number | null): ProblemFilterBuilder {
        this._minimumPoint = point;
        return this;
    }

    setMaximumPoint(point: number | null): ProblemFilterBuilder {
        this._maximumPoint = point;
        return this;
    }

    setHideAC(): ProblemFilterBuilder {
        this._hideAC = true;
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

    get hideAC() {
        return this._hideAC;
    }
}