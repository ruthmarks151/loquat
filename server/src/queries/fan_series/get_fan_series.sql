SELECT fan_sizes.fan_size_id, diameter, fan_sizes.fan_series_id, fan_type, outlet_area
            FROM fan_serieses
            JOIN fan_sizes
            ON fan_sizes.fan_series_id = fan_serieses.fan_series_id
            WHERE fan_serieses.fan_series_id = $1