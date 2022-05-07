import React from 'react'
import ReactDOM from 'react-dom'
import App from './App'
import getConfig from './config'
import { viewMethodOnContract } from './utils'
//import { initContract } from './utils'
import { data } from './hardcoded-data'
import { async } from 'regenerator-runtime'


// window.nearInitPromise = initContract()
//   .then(() => {
//     ReactDOM.render(
//       <App />,
//       document.querySelector('#root')
//     )
//   })
//   .catch(console.error)

async function initContractSolution(){
  const nearConfig = getConfig(process.env.NEAR_ENV || 'testnet');
  const solutionHash = await viewMethodOnContract(nearConfig, 'get_solution');
  return {data, solutionHash};
}

window.nearInitPromise = initContractSolution().then(({data, solutionHash})=>{
  ReactDOM.render(
    <App data={data} solutionHash={solutionHash}/>, document.getElementById('root')
  );

});
