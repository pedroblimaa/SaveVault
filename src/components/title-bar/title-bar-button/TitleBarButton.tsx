import './TitleBarButton.css'

interface TitleBarButtonProps {
  event: any
  icon: string
  alt: string
}

function TitleBarButton({ event, icon, alt }: TitleBarButtonProps) {
  return (
    <div className="titlebar-button" onClick={event}>
      <img src={icon} alt={alt} />
    </div>
  )
}

export default TitleBarButton
