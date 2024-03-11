// Fetch 0.0.0.0:3000 in parallel 
// to trigger a bug in the server
const s = await Promise.all(
  Array(1000).fill().map(() => fetch('http://0.0.0.0:3000', { verbose: true }))
);

console.log(s[0].status); // 200
