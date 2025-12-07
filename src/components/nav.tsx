import { useState } from "react"
import { Flex } from "antd"

export type NavConfig = Array<NavItem>

type NavItem = {
  title: string,
  id: string,
  page?: React.FC
}

type NavProps = {
  config: NavConfig
  className?: string
  style?: React.CSSProperties
  onActiveChange?: (activeId: string) => void
}

function Nav(prop: NavProps) {
  const [activeId, setActiveId] = useState('index')

  const handleClick = (id: string) => {
    setActiveId(id)
    if (prop.onActiveChange) {
      prop.onActiveChange(id)
    }
  }

  return (
    <Flex justify="center" align="center" gap={30}
      className={`p-2 ${prop.className || ''}`}
      style={prop.style}
    >
      {prop.config.map((item) => (
        <div className="font-bold font-size-1.4rem select-none"
          style={{
            color: activeId === item.id ? '#1890ff' : '#000'
          }}
          onClick={() => handleClick(item.id)}
        >
          {item.title}
        </div>
      ))
      }
    </Flex >
  )
}

export default Nav;