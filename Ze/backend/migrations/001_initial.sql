-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Ze counters table
CREATE TABLE ze_counters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255),
    description TEXT,
    initial_tau_z INTEGER NOT NULL DEFAULT 200,
    theta_z DOUBLE PRECISION NOT NULL DEFAULT 0.30,
    hilbert_dimension INTEGER NOT NULL DEFAULT 2,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Ze parameters table (global or counter-specific)
CREATE TABLE ze_parameters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ze_counter_id UUID REFERENCES ze_counters(id) ON DELETE CASCADE,
    parameter_name VARCHAR(100) NOT NULL,
    parameter_value DOUBLE PRECISION NOT NULL,
    parameter_unit VARCHAR(50),
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_param_name_per_counter UNIQUE (ze_counter_id, parameter_name)
);

-- Ze measurements table
CREATE TABLE ze_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ze_counter_id UUID NOT NULL REFERENCES ze_counters(id) ON DELETE CASCADE,
    measurement_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    v DOUBLE PRECISION NOT NULL, -- N_S/(N-1)
    n_s INTEGER NOT NULL, -- Number of S events
    n INTEGER NOT NULL, -- Total events (N)
    v_star_passive DOUBLE PRECISION DEFAULT 0.3069, -- 1 - ln(2)
    v_star_active DOUBLE PRECISION, -- Optional active optimum
    chi_ze DOUBLE PRECISION NOT NULL, -- Computed χ_Ze
    tau_z INTEGER NOT NULL, -- Current τ_Z
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_ze_parameters_counter_id ON ze_parameters(ze_counter_id);
CREATE INDEX idx_ze_measurements_counter_id ON ze_measurements(ze_counter_id);
CREATE INDEX idx_ze_measurements_time ON ze_measurements(measurement_time);

-- Insert default global parameters from PARAMETERS.md
INSERT INTO ze_parameters (parameter_name, parameter_value, parameter_unit, description) VALUES
    ('v*_passive', 0.3069, 'dimensionless', 'Theoretical optimum for passive counter: 1 - ln(2)'),
    ('v*_active', 0.456, 'dimensionless', 'Empirical optimum for active agent (preliminary)'),
    ('theta_Z', 0.30, 'dimensionless', 'Prediction threshold: fraction of T-events for decision'),
    ('theta_Q', 1.5, 'dimensionless', 'Quantum prediction threshold for EEG context'),
    ('alpha_S', 0.0, 'dimensionless', 'MCOA S-damage accumulation coefficient from events'),
    ('beta_S', 0.0, 'dimensionless', 'MCOA S-damage accumulation coefficient from time'),
    ('gamma_S', 0.0, 'dimensionless', 'MCOA interaction coefficient with other damage types (null hypothesis)'),
    ('hilbert_dimension', 2, 'dimensionless', 'Dimension of Hilbert space H for EEG context');

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply triggers to tables with updated_at
CREATE TRIGGER update_ze_counters_updated_at BEFORE UPDATE ON ze_counters
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_ze_parameters_updated_at BEFORE UPDATE ON ze_parameters
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();