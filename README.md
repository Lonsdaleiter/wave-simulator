Wave Simulator
=
The idea is to provide an interactive experience constructing
or configuring waves of sound or light in an open sandbox
environment with a sleek UI.

Consider the window as being split into two parts: UI and Sandbox.

The Wave Simulator can be thought of as a zero player game, like
a cellular automata: all that is required is the initial state.

The simulation has two states: on and off. When on, the user cannot
affect the state of the Sandbox except to stop or pause it. When off, the user
can affect the state of the Sandbox.

The Sandbox section is the world which the user modifies. The Sandbox
consists of objects which are tagged and from which the user can
fetch properties.

Note here that waves aren't taggable objects and cannot
exist in the Sandbox's initial state. They are generated
entities.

The UI section is where the user configures the initial state of
the Sandbox. This is done via command line. Commands:
- generate [object]
- modify [object tag] [property tag] to [value]
- add [behavior] to [object tag]
- modify [object tag] [behavior tag] to [behavior]
- go
- pause
- terminate
