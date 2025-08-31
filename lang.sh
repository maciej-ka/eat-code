#!/usr/bin/env node

const lang = [
  'C++',
  'Java',
  'Python',
  'Python 3',
  'C',
  'C#',
  'JavaScript',
  'TypeScript',
  'PHP',
  'Swift',
  'Kotlin',
  'Dart',
  'Go',
  'Ruby',
  'Scala',
  'Rust',
  'Racket',
  'Erlang',
  'Elixir'
]

const rand = (array) => {
  const i = Math.floor(Math.random() * array.length);
  return array[i];
};

console.log(rand(lang))
