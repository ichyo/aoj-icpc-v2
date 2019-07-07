import React, { useState, FormEvent, ChangeEvent } from 'react';
import { formatPoint } from '../utils';

export interface FormData {
    aojUserId: string | null,
    minimumPoint: number | null,
    maximumPoint: number | null,
    sinceYear: number | null,
    untilYear: number | null,
    hideAC: boolean,
    showPending: boolean,
}

interface FormProps {
    onSubmit: (data: FormData) => void,
    points: number[],
    years: number[],
}

const SearchForm: React.FC<FormProps> = ({ onSubmit, points, years }) => {
    const [aojUserId, setAojUserId] = useState("");
    const [minimumPoint, setMinimumPoint] = useState(null as number | null);
    const [maximumPoint, setMaximumPoint] = useState(null as number | null);
    const [hideAC, setHideAC] = useState(false);
    const [showPending, setShowPending] = useState(false);
    const [sinceYear, setSinceYear] = useState(null as number | null);
    const [untilYear, setUntilYear] = useState(null as number | null);

    const formData = (): FormData => {
        return {
            aojUserId,
            minimumPoint,
            maximumPoint,
            hideAC,
            showPending,
            sinceYear,
            untilYear,
        }
    };

    const handleSubmit = (event: FormEvent) => {
        event.preventDefault();
        const data = formData();
        onSubmit(data);
    };

    const handleMinChange = (event: ChangeEvent<HTMLSelectElement>) => {
        const value = event.target.value;
        const point = parseInt(value);
        if (!isNaN(point)) {
            setMinimumPoint(point);
        }
    };

    const handleMaxChange = (event: ChangeEvent<HTMLSelectElement>) => {
        const value = event.target.value;
        const point = parseInt(value);
        if (!isNaN(point)) {
            setMaximumPoint(point);
        }
    };

    const handleSinceChange = (event: ChangeEvent<HTMLSelectElement>) => {
        const value = event.target.value;
        const point = parseInt(value);
        if (!isNaN(point)) {
            setSinceYear(point);
        }
    };

    const handleUntilChange = (event: ChangeEvent<HTMLSelectElement>) => {
        const value = event.target.value;
        const point = parseInt(value);
        if (!isNaN(point)) {
            setUntilYear(point);
        }
    };

    const handleHideAcChange = (event: ChangeEvent<HTMLInputElement>) => {
        const value = event.target.checked;
        setHideAC(value);
    };

    const handleShowPendingChange = (event: ChangeEvent<HTMLInputElement>) => {
        const value = event.target.checked;
        setShowPending(value);
    };


    return (
        <form className="form-inline mb-3 mt-3" onSubmit={handleSubmit}>
            <input
                type="text"
                className="form-control mr-3 col-4 col-lg-3"
                placeholder="AOJ ID"
                value={aojUserId}
                onChange={e => setAojUserId(e.target.value)}
            />
            <select className="form-control col-2 col-lg-1" onChange={handleMinChange}>
                <option selected={minimumPoint == null}>FROM</option>
                {
                    points.map(p => <option selected={minimumPoint == p}>{formatPoint(p)}</option>)
                }
            </select>
            <select className="form-control col-2 col-lg-1" onChange={handleMaxChange}>
                <option selected={maximumPoint == null}>TO</option>
                {
                    points.map(p => <option selected={maximumPoint == p}>{formatPoint(p)}</option>)
                }
            </select>
            <span className="mr-3" />
            <select className="form-control col-2 col-lg-1" onChange={handleSinceChange}>
                <option selected={sinceYear == null}>SINCE</option>
                {
                    years.map(p => <option selected={sinceYear == p}>{p}</option>)
                }
            </select>
            <select className="form-control col-2 col-lg-1" onChange={handleUntilChange}>
                <option selected={untilYear == null}>UNTIL</option>
                {
                    years.map(p => <option selected={untilYear == p}>{p}</option>)
                }
            </select>
            <span className="mr-3" />
            <div className="form-check form-check-inline">
                <input className="form-check-input" id="hideAC" type="checkbox" checked={hideAC} onChange={handleHideAcChange} />
                <label className="form-check-label" htmlFor="hideAC">Hide AC</label>
            </div>
            <div className="form-check form-check-inline">
                <input className="form-check-input" id="showPending" type="checkbox" checked={showPending} onChange={handleShowPendingChange} />
                <label className="form-check-label" htmlFor="showPending">Show ?</label>
            </div>
            <span className="mr-2" />
            <button type="submit" className="btn btn-primary">Update</button>
        </form>
    );
}

export default SearchForm;
