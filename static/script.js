let current_time = Math.floor(+(new Date())/1000);
let period = 60*60*24;

let a=getStartOfDayTimestamp();
let b=current_time;

function getStartOfDayTimestamp() {
  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth();
  const date = now.getDate();

  const sod = new Date(year, month, date, 0, 0, 0);
  return Math.floor(sod.getTime() / 1000);
}

function changegraph(time) {
  time += 60*60*24;
  a=time - period;
  b=time;
  clearInterval(interval);
  get_chart();
}

async function getData() {

  switch (document.getElementById("period").value) {
    case "today":
      let current_time = Math.floor(+(new Date())/1000);
      a=getStartOfDayTimestamp();
      b=current_time;
      clearInterval(interval);
      get_chart();
      return;
    case "1-hour":
      period = 60*60;
      break;
    case "6-hours":
      period = 60*60*6;
      break;
    case "12-hours":
      period = 60*60*12;
      break;
    case "24-hours":
      period = 60*60*24;
      break;
    case "weekly":
      period = 60*60*24*7;
      break;
    case "monthly":
      period = 60*60*24*30;
      break;
    case "yearly":
      period = 60*60*24*365;
      break;
    default:
      break;
  }

  a=current_time - period;
  b=current_time;

  clearInterval(interval);
  get_chart();
}

async function get_chart() {
  let start_time = a;
  let end_time = b;

  {
    const response = await fetch("/timeline?width=" + window.innerWidth);
    const text = await response.text();
    document.getElementById("timeline").innerHTML = text;
  }

  const response = await fetch("/sankey?start_time=" + start_time + "&end_time=" + end_time + "&width=" + window.innerWidth + "&height=" + window.innerHeight);
  const text = await response.text();
  document.getElementById("app").innerHTML = text;

  const hoverElements = document.querySelectorAll('.hover-element');

  const tooltip = document.getElementById('tooltip');

  hoverElements.forEach((element) => {
    element.addEventListener('mouseover', (event) => {
      element.classList.add('hover-highlight');
      tooltip.innerHTML = element.getAttribute('data-tooltip');
      tooltip.style.display = 'block';
    });

    element.addEventListener('mouseout', () => {
      element.classList.remove('hover-highlight');
      tooltip.style.display = 'none';
    });

    element.addEventListener('mousemove', (event) => {
      tooltip.style.top = `${event.pageY + 10}px`;
      tooltip.style.left = `${event.pageX + 10}px`;
    });
  });
}

get_chart();
let interval = setInterval(function() {
  a = getStartOfDayTimestamp();
  b = Math.floor(+(new Date())/1000);
  get_chart();
}, 1000);