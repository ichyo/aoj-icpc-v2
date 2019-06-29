import React, { useState, FormEvent, ChangeEvent } from 'react';
import { POINT_CONVERSION_COMPRESSED } from 'constants';

export interface FormData {
    aojUserId: string | null,
    minimumPoint: number | null,
    maximumPoint: number | null,
}

interface FormProps {
    onSubmit: (data: FormData) => void,
    points: number[],
}

const SearchForm: React.FC<FormProps> = ({ onSubmit, points }) => {
    const [aojUserId, setAojUserId] = useState("");
    const [minimumPoint, setMinimumPoint] = useState(null as number | null);
    const [maximumPoint, setMaximumPoint] = useState(null as number | null);

    const formData = (): FormData => {
        return {
            aojUserId,
            minimumPoint,
            maximumPoint,
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
                    points.map(p => <option selected={minimumPoint == p}>{p}</option>)
                }
            </select>
            <select className="form-control col-2 col-lg-1" onChange={handleMaxChange}>
                <option selected={maximumPoint == null}>TO</option>
                {
                    points.map(p => <option selected={maximumPoint == p}>{p}</option>)
                }
            </select>
            <span className="mr-3" />
            <button type="submit" className="btn btn-primary">Update</button>
        </form>
    );
}

export default SearchForm;