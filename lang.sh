#!/usr/bin/env node

const langs = [
  "Bash (sh)",
  "C (c)",
  "C# (cs)",
  "C++ (cpp)",
  "Dart (dart)",
  "Elixir (ex)",
  "Erlang (erl)",
  "Go (go)",
  "Haskell (hs)",
  "Java (java)",
  "JavaScript (js)",
  "Kotlin (kt)",
  "PHP (php)",
  "Python 2 (py2)",
  "Python 3 (py3)",
  "Racket (rkt)",
  "Ruby (rb)",
  "Rust (rs)",
  "Scala (scala)",
  "Swift (swift)",
  "TypeScript (ts)",
]

const rand = (array) => {
  const i = Math.floor(Math.random() * array.length);
  return array[i];
};

console.log(rand(langs))
