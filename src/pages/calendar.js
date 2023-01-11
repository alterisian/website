/* global FullCalendar */
; (() => {
  const styleElm = document.createElement('style');
  document.head.append(styleElm);
  styleElm.sheet.insertRule('.fc .fc-toolbar .fc-toolbar-title { font-size: inherit }')
  styleElm.sheet.insertRule('.fc .fc-toolbar.fc-header-toolbar { margin-bottom: 0.5em; }')
  styleElm.sheet.insertRule('.fc .fc-timegrid-slot { height: 2.5em; }')
  styleElm.sheet.insertRule('.fc .fc-event-time { white-space: normal; }')
  styleElm.sheet.insertRule('.fc .fc-event-main-frame { overflow-wrap: break-word; }')
  const calendarEl = document.querySelector('div:empty')
  const calendar = new FullCalendar.Calendar(calendarEl, {
    initialView: 'timeGridWeek',
    slotDuration: '01:00:00',
    expandRows: true,
    dayHeaderFormat: { weekday: 'short' },
    views: {
      timeGridWeek: {
        allDaySlot: false
      }
    },
    events,
    height: 'auto',
    contentHeight: 'auto',
    eventMinHeight: 40,
    nowIndicator: true,
    eventTimeFormat: {
      hour: 'numeric',
      minute: '2-digit',
      omitZeroMinute: true,
      meridiem: 'narrow',
    },
    eventBorderColor: 'transparent'
  })
  calendar.render()
  document.querySelector('.fc-toolbar-title').prepend(
    `Time zone: ${Intl.DateTimeFormat().resolvedOptions().timeZone}`,
    document.createElement('br'),
  )
})()
