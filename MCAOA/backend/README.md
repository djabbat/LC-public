# MCAOA Backend

Multi-Counter Architecture of Organismal Aging backend service.

## Overview

This is the backend service for the MCAOA (Multi-Counter Architecture of Organismal Aging) subproject of LC. It provides a REST API for managing the five canonical aging counters, tissues, subjects, damage measurements, and computing tissue loads according to the MCAOA framework.

## Features

- **Counter Management**: CRUD operations for the five canonical counters (telomere, centriolar polyglutamylation, mitochondrial ROS, epigenetic drift, proteostasis collapse)
- **Tissue Management**: Manage tissue types with a-priori weighting functions
- **Subject Tracking**: Track organisms (mice, humans) being studied
- **Damage Measurements**: Store and retrieve counter damage values