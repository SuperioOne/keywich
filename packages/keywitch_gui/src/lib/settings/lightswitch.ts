import { modeCurrent, setModeUserPrefers, setModeCurrent } from '@skeletonlabs/skeleton';

function createLightswitch()
{
  const { subscribe, set, update } = modeCurrent;

  const setMode = (value: boolean) =>
  {
    setModeUserPrefers(value);
    setModeCurrent(value);
  };

  return {
    subscribe,
    switchToDark: () => { set(false); setMode(false); },
    switchToLight: () => { set(true); setMode(true); },
    flip: () =>
    {
      update((value) =>
      {
        const newValue = !value;
        setMode(newValue);
        return newValue;
      });
    },
    reset: () => { set(true); setMode(true); }
  };
}

export const lightSwitchController = createLightswitch();