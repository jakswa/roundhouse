ready(async function() {
  let geoPermissionState = await navigator.permissions
    .query({ name: 'geolocation' })
    .then((status) => status.state);
  let gavePermission = geoPermissionState === 'granted';
  let positionSet = document.cookie.indexOf('nearby_to') > -1;
  let nearbyOn = document.cookie.indexOf('nearby_on') > -1;

  if (nearbyOn && gavePermission && !positionSet) attemptPosition();

  let ele = document.querySelector('#nearby-stations');

  ele.addEventListener('click', () => {
    if (nearbyOn) turnNearbyOff(ele)
    else {
      noNearby = false;
      document.cookie = 'nearby_on=1;max-age=31536000';
      attemptPosition(new Date());
      ele.classList.remove('opacity-50');
    }
  });
});

function turnNearbyOff(ele) {
    document.cookie = 'nearby_to=; Max-Age=0';
    document.cookie = 'nearby_on=; Max-Age=0';
    positionSet = false;
    noNearby = true;
    ele.classList.add('opacity-50');
    document.querySelector('#nearby-stations-list').innerHTML = '';
}

function ready(fn) {
  if (document.readyState !== 'loading') {
    fn();
  } else {
    document.addEventListener('DOMContentLoaded', fn);
  }
}

function attemptPosition(attemptedAt) {
  navigator.geolocation.getCurrentPosition(
    (position) => {
      document.cookie = `nearby_to=${position.coords.latitude},${position.coords.longitude}`;
      decideAJAX(new Date() - attemptedAt);
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

function decideAJAX(_locationDelay) {
  let ele = document.querySelector('#station-list');
  ele.setAttribute('hx-trigger', 'intersect once');
  ele.setAttribute('hx-get', '/');
  ele.setAttribute('hx-swap', 'morph');
  htmx.process(ele);
}
