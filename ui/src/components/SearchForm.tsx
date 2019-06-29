import React, { useState, FormEvent } from 'react';

export interface FormData {
    aojUserId: string,
}

interface FormProps {
    onSubmit: (data: FormData) => void,
}

const SearchForm: React.FC<FormProps> = ({ onSubmit }) => {
    const [aojUserId, setAojUserId] = useState("");

    const formData = (): FormData => {
        return {
            aojUserId
        }
    };

    const handleSubmit = (event: FormEvent) => {
        event.preventDefault();
        const data = formData();
        onSubmit(data);
    };

    return (
        <form className="form-inline mb-3 mt-3" onSubmit={handleSubmit}>
            <input
                type="text"
                className="form-control mr-2 col-4 col-md-3"
                placeholder="AOJ ID"
                value={aojUserId}
                onChange={e => setAojUserId(e.target.value)}
            />
            <button type="submit" className="btn btn-primary">Update</button>
        </form>
    );
}

export default SearchForm;