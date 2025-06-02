# Rusty K8s

These are high-level API bindings to the client K8s resources.

The bindings are generated from Protobuf files in the official K8s repositories.

There is a proc macro that generates helper methods, fields, etc. for QOL.

The goal of this repository is to use it for K8s resource configuration in lieu of something like Helm
for full type-checking at compile time. It should also allow for more maintainable config over time as
you can define traits for resources and ensure things are implemented across all trait boundaries.
