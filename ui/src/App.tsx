import React, { useState, useEffect } from 'react';
import './App.css';
import './components/ProblemTable';
import ProblemTable from './components/ProblemTable';
import SearchForm, { FormData } from './components/SearchForm';

const App: React.FC = () => {
  const [problems, setProblems] = useState([]);
  const [user, setUser] = useState(null);

  useEffect(() => {
    fetch("/api/v1/problems")
      .then(res => res.json())
      .then(res => setProblems(res));
  }, []);

  const handleSubmit = (data: FormData) => {
    if (data.aojUserId) {
      fetch("/api/v1/aoj_users/" + data.aojUserId)
        .then(res => res.json())
        .then(res => setUser(res), err => console.log(err)); // TODO: error handling
    }
  };

  return (
    <div className="container">
      <SearchForm onSubmit={handleSubmit} />
      <ProblemTable problems={problems} user={user} />
    </div>
  );
}

export default App;
