# Handover Notes

This branch is an experiment to move the vehicle with impulse forces on the body rather than through rotating wheels.

* Wheels no longer have motors, but they can rotate freely.
* The control system adds an external impulse deried from the forward or back vector of the body transform.
* Steering is unchanged.
* Joints are now part of an ImpulseJointSet (not MultibodyJointSet)

I believe this approach could work but there are currently some blocking problems.

For reasons beyond my comprehension, external impulses have absolutely no effect if the entity is attached via a
multibody joint. The vehicle will sit stationary with impulse having literally no effect on the simulation.
Changing impulse strength, friction, mass, density, damping, or anything else doesn't fix it; the
multibody joint is the problem.

Changing all joints to impulse joint does make the external impulse work, but that means dealing with having wheels
attached by impulse joints, resulting in a pathetic clown car of a simulation where wheels are attached by extremely
persistent, but extremely bad, elastic.

I think something from this could be the eventual answer to dynamic vehicles in Bevy but right now the joint system
is much too fragile.

