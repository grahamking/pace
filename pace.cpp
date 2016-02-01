/*
 * Copyright 2016 Graham King
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * For full license details see <http://www.gnu.org/licenses/>.
 */

#include <iostream>
#include <string>
#include <cmath>
#include <iomanip>

using namespace std;

void usage() {
  cout << "pace has two modes: pace and distance.\n";
  cout << "DISTANCE MODE: `pace 10k 1h`\n";
  cout << "Usage: pace [distance] [time]\n";
  cout << "distance:\n";
  cout << "\tnumber followed by 'k' for kilometers, e.g. 10k\n";
  cout << "\tnumber followed by 'm' for miles, e.g. 26.2m\n";
  cout << "\tspecial word 'marathon' or 'half'\n";
  cout << "time:\n";
  cout << "\tnumber followed by 'h' for hours\n";
  cout << "\tnumber followed by 'm' for minutes\n";
  cout << "PACE MODE: `pace 4:30k`\n";
  cout << "Usage: pace [pace]\n";
  cout << "pace:\n";
  cout << "\tmin:secs followed by 'k' for per kilometer, e.g. 5:30k\n";
  cout << "\tmins:secs followed by 'm' for per mile, e.g. 7:00m\n";
}

string fmt_time(float raw_time) {
  if (raw_time < 60) {
    return to_string(int(raw_time)) + 'm';
  }

  float h = raw_time / 60;
  int hours = int(floor(h));
  string out = to_string(hours) + 'h';
  int minutes = int(60 * (h - hours));
  if (minutes == 0) {
    return out;
  }
  if (minutes < 10) {
    out += '0' + to_string(minutes);
  } else {
    out += to_string(minutes);
  }
  return out;
}

void display_distances(float pace) {
  cout << "At that pace:\n";
  cout << "\tMarathon:\t" << fmt_time(42.2 * pace) << '\n';
  cout << "\tHalf-Marathon:\t" << fmt_time(21.1 * pace) << '\n';
  cout << "\t10k:\t\t" << fmt_time(10 * pace) << '\n';
  cout << "\t5k:\t\t" << fmt_time(5 * pace) << '\n';
}

int do_distance(string d_raw, string t_raw) {

  if (d_raw == "marathon") {
    d_raw = "42.2k";
  } else if (d_raw == "half") {
    d_raw = "21.1k";
  }

  char dist_unit = d_raw[d_raw.size()-1];
  float dist_k = 0, dist_m = 0;
  if (dist_unit == 'k') {
    dist_k = stof(d_raw);
    dist_m = dist_k * 0.62137119;
  } else if (dist_unit == 'm') {
    dist_m = stof(d_raw);
    dist_k = dist_m * 1.609;
  } else {
    cout << "Unknown distance unit: " << dist_unit << ". Must be k or m\n";
    return 1;
  }

  char time_unit = t_raw[t_raw.size()-1];
  float time = stof(t_raw);
  if (time_unit == 'h') {
    time = time * 60;
  } else if (time_unit == 's') {
    time = time / 60;
  } else if (time_unit != 'm') {
    cout << "Invalid time unit '" << time_unit << "'. Must be h, m or s\n";
    return 1;
  }

  float result_k = time / dist_k;
  int min_k = int(floor(result_k));
  int secs_k = int(60 * (result_k - min_k));

  float result_m = time / dist_m;
  int min_m = int(floor(result_m));
  int secs_m = int(60 * (result_m - min_m));

  cout << setfill('0') << fixed << setprecision(1);
  cout << dist_k << " km / " << dist_m << " miles in " << t_raw << ":";
  cout << " " << min_k << ':' << setw(2) << secs_k << "/km";
  cout << ", " << min_m << ':' << setw(2) << secs_m << "/mile";
  cout << '\n';

  display_distances(result_k);
  return 0;
}

float convert_to_per_km(float per_mile) {
  return per_mile * 0.62137119223733;
}

int do_pace(string p) {
  int colon = p.find_first_of(':');
  float minutes = stof(p.substr(0, colon));
  float seconds = stof(p.substr(colon+1));
  minutes += (seconds / 60);

  char dist_unit = p[p.size()-1];
  if (dist_unit == 'm') {
    minutes = convert_to_per_km(minutes);
  } else if (dist_unit != 'k') {
    cout << "Invalid pace unit '" << dist_unit << "'. Must be 'k' or 'm'\n";
    return 1;
  }

  display_distances(minutes);
  return 0;
}

int main(int argc, char **argv) {
  if (argc != 2 && argc != 3) {
    usage();
    return 1;
  }

  if (argc == 2) {
    return do_pace(argv[1]);
  } else if (argc == 3) {
    return do_distance(argv[1], argv[2]);
  }
}
