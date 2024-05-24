let positionSet = document.cookie.indexOf('nearby_to') > -1;
let nearbyOn = document.cookie.indexOf('nearby_on') > -1;

ready(async function() {
  let geoPermissionState = await navigator.permissions
    .query({ name: 'geolocation' })
    .then((status) => status.state);
  let permissionUnblocked = geoPermissionState !== 'denied';

  if (nearbyOn && permissionUnblocked && !positionSet) attemptPosition();

  let ele = document.querySelector('#nearby-stations');
  document.addEventListener('click', (event) => {
    if (!event.target.closest('#nearby-stations')) return;

    if (nearbyOn) turnNearbyOff(ele)
    else {
      nearbyOn = true;
      document.cookie = 'nearby_on=1;max-age=31536000';
      attemptPosition();
      ele.classList.remove('opacity-50');
      ele.querySelector('.handle').classList.add('translate-x-[100%]');
    }
  });
});

function turnNearbyOff(ele) {
  document.cookie = 'nearby_to=; Max-Age=0';
  document.cookie = 'nearby_on=; Max-Age=0';
  positionSet = false;
  nearbyOn = false;
  ele.classList.add('opacity-50');
  ele.querySelector('.handle').classList.remove('translate-x-[100%]');
  document.querySelector('#nearby-stations-list').innerHTML = '';
}

function ready(fn) {
  if (document.readyState !== 'loading') {
    fn();
  } else {
    document.addEventListener('DOMContentLoaded', fn);
  }
}

function attemptPosition() {
  navigator.geolocation.getCurrentPosition(
    (position) => {
      document.cookie = `nearby_to=${position.coords.latitude},${position.coords.longitude}`;
      decideAJAX();
    },
    (err) => {
      let ele = document.querySelector('#nearby-stations')
      if (err.code === err.PERMISSION_DENIED) {
        geoFeedback('denied', ele);
      } else {
        geoFeedback('geolocation error', ele);
      }
      turnNearbyOff(ele);
    },
    { maximumAge: 60000 }
  );
}

function geoFeedback(msg, ele) {
  let em = document.createElement('em');
  em.innerHTML = `(${msg})`;
  ele.appendChild(em);
}

function decideAJAX() {
  let ele = document.querySelector('#station-list');
  ele.setAttribute('hx-trigger', 'intersect once');
  ele.setAttribute('hx-get', ''); // blank url means "fetch current url"
  ele.setAttribute('hx-swap', 'morph');
  htmx.process(ele);
}
