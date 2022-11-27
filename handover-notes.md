# Handover Notes

An experiment to simulate a vehicle as a sphere. External impulses are applied to directly to the sphere and the
apparent vehicle position would be derived from the sphere's.

* Vehicle colliders have all been removed.
* Instead the collider is a sphere.
* There are no motors. Movement is a force directly applied to the sphere.

From a simulation perspective, this is definitely the most stable experiment yet.

The problem is everything else. Controls are unpredictable because of the sphere rotation. On a similar note,
it's difficult putting some physics-less avatar in the right position because the sphere has no true facing.

I suspect with proper understanding of quaternions and local vectors this could be made to work, but that would
take me a lot of reading only to provide a facsimilie simulation of what a rectangular vehicle actually behaves like.
Note that this approach doesn't cleanly deal with things like the vehicle being on a sloping. With that in mind, I've
bravely decided to abandon the venture.

