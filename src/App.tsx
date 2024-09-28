import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './App.css'
import TitleBar from './components/title-bar/TitleBar'
import Home from './pages/home/Home'
import Settings from './pages/settings/Settings'

function App() {
  return (
    <BrowserRouter>
      <div>
        <TitleBar></TitleBar>
        <div className='app-content'>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </div>
      </div>
    </BrowserRouter>
  )
}

export default App
