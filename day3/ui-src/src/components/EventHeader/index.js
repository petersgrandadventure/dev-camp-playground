import React from 'react'
import style from './index.module.css'

export const EventHeader = ({
  state: { event, user },
  actions: { joinEvent, leaveEvent }
}) => {
  const attendingCurrent = event.users ? event.users.find(x => x === user.id) : false
  return (
    <header className={style.component}>
      {attendingCurrent ?
        <div>You're Attending</div>
        : <div onClick={() => joinEvent(event)}>Attend</div>}
      <row->
        <h1>{event.name && event.name.replace(user.id, '')}</h1>
      </row->
      {event.users && (
        <div>
          <span>{event.users.length}</span>
          <svg id="members" viewBox="0 0 24 24">
            <path d="M9 11.75c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zm6 0c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8 0-.29.02-.58.05-.86 2.36-1.05 4.23-2.98 5.21-5.37C11.07 8.33 14.05 10 17.42 10c.78 0 1.53-.09 2.25-.26.21.71.33 1.47.33 2.26 0 4.41-3.59 8-8 8z"/>
            <path d="M0 0h24v24H0z" fill="none"/>
          </svg>
        </div>
      )}
    </header>
  )
}