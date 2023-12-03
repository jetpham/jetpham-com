import background from './images/Vase of Flowers.webp'
import './App.css'

function App() {
  return (
    <div className='App'>
      <img src={background} className='background-img' alt='flowers' />
      <header className='header'>
        <h1 className='title'>JET PHAM</h1>
      </header>
    </div>
  )
}

export default App
