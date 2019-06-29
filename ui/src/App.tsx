import React, { useState, useEffect } from 'react';
import './App.css';
import './components/ProblemTable';
import ProblemTable from './components/ProblemTable';
import ProblemFilter from './ProblemFilter';
import SearchForm, { FormData } from './components/SearchForm';
import { Problem, User } from './model';


const App: React.FC = () => {
  const [problems, setProblems] = useState([] as Problem[]);
  const [user, setUser] = useState(null as User | null);
  const [problemFilter, setProblemFilter] = useState(ProblemFilter.default());

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

    const builder = ProblemFilter.builder()
      .setMinimumPoint(data.minimumPoint)
      .setMaximumPoint(data.maximumPoint);

    const filter = builder.build();
    setProblemFilter(filter);
  };

  const points = Array.from(new Set(problems.map(p => p.point).sort((a, b) => a - b)));
  const filteredProblems = problemFilter.filters(problems, user);

  return (
    <div className="container">
      <SearchForm onSubmit={handleSubmit} points={points} />
      <ProblemTable problems={filteredProblems} user={user} />
    </div>
  );
}

export default App;
