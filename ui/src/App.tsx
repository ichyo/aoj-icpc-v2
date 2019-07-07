import React, { useState, useEffect } from 'react';
import './App.css';
import './components/ProblemTable';
import ProblemTable from './components/ProblemTable';
import ProblemFilter from './ProblemFilter';
import SearchForm, { FormData } from './components/SearchForm';
import { Problem, User } from './model';


const App: React.FC = () => {
  const [problems, setProblems] = useState([] as Problem[]);
  const [solutions, setSolutions] = useState(new Map());
  const [user, setUser] = useState(null as User | null);
  const [problemFilter, setProblemFilter] = useState(ProblemFilter.default());

  useEffect(() => {
    const apiProblems = fetch("/api/v1/problems")
      .then(res => res.json());

    const apiSolutions = fetch("/api/v1/problems/solutions")
      .then(res => res.json());

    Promise.all([apiProblems, apiSolutions])
      .then(([problems, solutions]) => {
        problems.sort((a: Problem, b: Problem) => {
          if (a.point != b.point) {
            return a.point - b.point
          }
          return (solutions[b.id.toString()] || 0)
            - (solutions[a.id.toString()] || 0);
        })
        setProblems(problems);
        setSolutions(new Map(Object.entries(solutions)));
      });

    //.then(res => setSolutions(new Map(Object.entries(res))));
  }, []);

  const handleSubmit = (data: FormData) => {
    if (data.aojUserId) {
      fetch("/api/v1/aoj_users/" + data.aojUserId)
        .then(res => res.json())
        .then(res => setUser(res), err => console.log(err)); // TODO: error handling
    }

    const builder = ProblemFilter.builder()
      .setMinimumPoint(data.minimumPoint)
      .setMaximumPoint(data.maximumPoint)
      .setHideAC(data.hideAC);

    const filter = builder.build();
    setProblemFilter(filter);
  };

  const handleSort = (comparator: (a: Problem, b: Problem) => number) => {
    const sorted_problems = Array.from(problems.sort(comparator));
    setProblems(sorted_problems);
  }

  const points = Array.from(new Set(problems.map(p => p.point).sort((a, b) => a - b)));
  const filteredProblems = problemFilter.filters(problems, user);

  return (
    <div className="container">
      <SearchForm onSubmit={handleSubmit} points={points} />
      <ProblemTable
        problems={filteredProblems}
        user={user}
        solutions={solutions}
        handleSort={handleSort}
      />
    </div>
  );
}

export default App;
