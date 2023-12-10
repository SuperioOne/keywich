import {
  modeCurrent,
  setModeUserPrefers,
  setModeCurrent,
} from '@skeletonlabs/skeleton';

function createLightswitch() {
  const {subscribe, set, update} = modeCurrent;

  const setMode = (value: boolean) => {
    set(value);
    setModeUserPrefers(value);
    setModeCurrent(value);
  };

  return {
    subscribe,
    switchToDark: () => {
      setMode(false);
    },
    switchToLight: () => {
      setMode(true);
    },
    set: setMode,
    flip: () => {
      update((value) => {
        const newValue = !value;
        setMode(newValue);
        return newValue;
      });
    },
    reset: () => {
      setMode(true);
    }
  };
}

export const lightSwitchController = createLightswitch();