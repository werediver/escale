pkg load signal;

% function b = design_fir(f_s, f1, f2, k_order = 1)
function b = design_fir(f_s, f1, f2, order)
  % [order, band_trans, kaiser_beta, filter_type] = kaiserord(
  %   [f1, f2], [1, 0], [0.05, 0.05], f_s
  % );
  % % b1 = fir1(order, band_trans, filter_type, kaiser(order + 1, kaiser_beta));
  % order = 2 * fix((k_order * order + 1) / 2); % Make sure the order is even

  b = firls(
    order,
    [0, 2 * f1 / f_s, 2 * f2 / f_s, 1],
    [1, 1, 0, 0]
  );
  b = b / sum(b); % Make sure DC gain is 1

  name = sprintf("%.2f–%.2f Hz (%.2f oct.) / %.2f Hz, order %d\n", f1, f2, log10(f2 / f1) / log10(2), f_s, order);
  printf("# %s", name);

  % An ad-hoc stop-band attenuation definition
  [h, f] = freqz(b, 1, 4096, f_s);
  h_mag = abs(h);
  first_stop_band_peak_idx = find(
    and(
      f(1 : end - 1) >= f2,
      diff(h_mag) >= 0
    )
  )(1);
  a_stop = max(h_mag(first_stop_band_peak_idx : end));
  f_stop = f(find(h_mag <= a_stop)(1));
  printf("Rejection at least %.2f dB (×%.2f) at f ≥ %.2f Hz\n", 20 * log10(a_stop), a_stop, f_stop);

  % printf("b = [")
  % printf("%f, ", b);
  % printf("]\n");

  printf("\n");

  % figure();
  % freqz(b, 1, 4096, f_s);
  % legend(name);

  step = ones(1, 2 * order);
  y = filter(b, 1, step);
  plot(y, "DisplayName", name);
endfunction

f_s = 320;
% f1 = 0.25;
% band_transition_width_oct = 2;

hold on;
for f1 = 1e-3
  % for band_transition_width_oct = 2
  %   f2 = f1 * 2^(band_transition_width_oct - 1);
  for f2 = 1.8
    % for k_order = 1.5
    for order = 320
      design_fir(f_s, f1, f2, order);
    endfor
  endfor
endfor

% Single-pole IIR / Exponential Moving Average
step = ones(1, 2 * order);
decay = 0.9;
y = filter([1 - decay], [1, -decay], step);
plot(y, "DisplayName", sprintf("EMA(decay=%.2f)", decay));

legend();
hold off;

